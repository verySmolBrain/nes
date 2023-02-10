use crate::emulator::cpu::Cpu;
use crate::emulator::bus::Bus;

pub const RAM: u16 = 0x0000;
pub const RAM_MIRRORS_END: u16 = 0x1FFF;

pub const PPU_REGISTERS_MIRRORS_START: u16 = 0x2008;
pub const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub const ROM: u16 = 0x8000;
pub const ROM_MIRRORS_END: u16 = 0xFFFF;

pub const PPU_CONTROLLER: u16 = 0x2000;
pub const PPU_MASK: u16 = 0x2001;
pub const PPU_STATUS: u16 = 0x2002;
pub const PPU_OAM_ADDR: u16 = 0x2003;
pub const PPU_OAM_DATA: u16 = 0x2004;
pub const PPU_SCROLL: u16 = 0x2005;
pub const PPU_ADDRESS: u16 = 0x2006;
pub const PPU_DATA: u16 = 0x2007;
pub const OAM_DMA: u16 = 0x4014;

pub const JOYPAD_1: u16 = 0x4016;

pub trait Mem {
    fn mem_read(&mut self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, value: u8);

    fn mem_read_u16(&mut self, addr: u16) -> u16 {
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
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM ..= RAM_MIRRORS_END => {
                self.cpu_vram[(addr & 0b111_11111111) as usize]
            },
            JOYPAD_1 => self.joypad.read(),
            
            PPU_STATUS => self.ppu.read_status(),
            PPU_OAM_DATA => self.ppu.read_oam_data(),
            PPU_DATA => self.ppu.read_data(),

            PPU_REGISTERS_MIRRORS_START ..= PPU_REGISTERS_MIRRORS_END => {
                self.mem_read(addr & 0b00100000_00000111) //addr % 0x2000
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
            JOYPAD_1 => self.joypad.write(data),

            PPU_CONTROLLER => self.ppu.write_controller(data),
            PPU_MASK => self.ppu.write_mask(data),
            PPU_OAM_ADDR => self.ppu.write_oam_addr(data),
            PPU_OAM_DATA => self.ppu.write_oam_data(data),
            PPU_SCROLL => self.ppu.write_scroll(data),
            PPU_ADDRESS => self.ppu.write_address(data),
            PPU_DATA => self.ppu.write_data(data),

            PPU_REGISTERS_MIRRORS_START ..= PPU_REGISTERS_MIRRORS_END => {
                self.mem_write(addr & 0b00100000_00000111, data) //addr % 0x2000
            },

            OAM_DMA => {
                let mut buffer = [0u8; 256];
                let hi: u16 = (data as u16) << 8;
                for i in 0..256 {
                    buffer[i] = self.mem_read(hi + i as u16);
                }

                self.ppu.write_oam_dma(&buffer);
            }

            0x8000 ..= 0xFFFF => {
                panic!("Attempted to write to ROM")
            },
            _ => {}
        }
    }
}

impl Mem for Cpu {
    fn mem_read(&mut self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
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