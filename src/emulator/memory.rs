use crate::emulator::cpu::Cpu;
use crate::emulator::bus::Bus;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;

// const PPU_REGISTERS: u16 = 0x2000;
// const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

const ROM: u16 = 0x8000;
const ROM_MIRRORS_END: u16 = 0xFFFF;

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, value: u8);

    fn mem_read_u16(&self, addr: u16) -> u16 {
        u16::from_le_bytes([ // LE
            self.mem_read(addr),
            self.mem_read(addr + 1)
        ])
    }

    fn mem_write_u16(&mut self, addr: u16, value: u16) {
        value.to_le_bytes().iter().enumerate().for_each(|(i, v)| {
            self.mem_write(addr + i as u16, *v) // LE
        })
    }
}

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM ..= RAM_MIRRORS_END => {
                self.cpu_vram[(addr & 0b111_11111111) as usize]
            },
            ROM ..= ROM_MIRRORS_END => {
                self.read_rom(addr)
            },
            _ => 0
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM ..= RAM_MIRRORS_END => {
                self.cpu_vram[(addr & 0b111_11111111) as usize] = data;
            },
            0x8000 ..= 0xFFFF => {
                panic!("Attempted to write to ROM")
            },
            _ => {}
        }
    }
}

impl Mem for Cpu {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        self.bus.mem_read_u16(pos)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.bus.mem_write_u16(pos, data)
    }
}



const STACK: u16 = 0x0100;

pub trait Stack {
    fn stack_push_u16(&mut self, value: u16);
    fn stack_pop_u16(&mut self) -> u16;
    fn stack_pop_u8(&mut self) -> u8;
    fn stack_push_u8(&mut self, value: u8);
}

impl Stack for Cpu {
    fn stack_push_u16(&mut self, value: u16) {
        value.to_be_bytes().iter().for_each(|v| {
            self.stack_push_u8(*v)
        })
    }

    fn stack_pop_u16(&mut self) -> u16 {
        u16::from_le_bytes([ // Since we push in LE, we need to pop in BE
            self.stack_pop_u8(),
            self.stack_pop_u8(),
        ])
    }

    fn stack_pop_u8(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(STACK + self.stack_pointer as u16)
    }

    fn stack_push_u8(&mut self, value: u8) {
        self.mem_write(STACK + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
}