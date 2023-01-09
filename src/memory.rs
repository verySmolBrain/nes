use crate::cpu::CPU;
use crate::bus::Bus;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;

// const PPU_REGISTERS: u16 = 0x2000;
// const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

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
            }
            _ => 0
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM ..= RAM_MIRRORS_END => {
                self.cpu_vram[(addr & 0b111_11111111) as usize] = data;
            }
            _ => {}
        }
    }
}

impl Mem for CPU {
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