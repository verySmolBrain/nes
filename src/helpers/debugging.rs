use crate::emulator::cpu::Cpu;
use crate::emulator::ppu::Ppu;
use crate::emulator::bus::Bus;
use crate::emulator::joypad::Joypad;
use crate::emulator::memory::{ RAM, RAM_MIRRORS_END, 
    PPU_REGISTERS_MIRRORS_START, PPU_REGISTERS_MIRRORS_END, ROM, 
    ROM_MIRRORS_END, PPU_STATUS, PPU_OAM_DATA, PPU_DATA, JOYPAD_1 };

impl Cpu {
    pub fn mem_read_debugging(&self, addr: u16) -> u8 {
        self.bus.mem_read_debugging(addr)
    }

    pub fn mem_read_debugging_u16(&mut self, addr: u16) -> u16 {
        u16::from_le_bytes([ // LE
            self.mem_read_debugging(addr),
            self.mem_read_debugging(addr + 1)
        ])
    }
}

impl Bus {
    pub fn mem_read_debugging(&self, addr: u16) -> u8 {
        match addr {
            RAM ..= RAM_MIRRORS_END => {
                self.cpu_vram[(addr & 0b111_11111111) as usize]
            },
            JOYPAD_1 => self.joypad.read_debugging(),
            
            PPU_STATUS => self.ppu.read_status_debugging(),
            PPU_OAM_DATA => self.ppu.read_oam_data(),
            PPU_DATA => self.ppu.read_data_debugging(),

            PPU_REGISTERS_MIRRORS_START ..= PPU_REGISTERS_MIRRORS_END => {
                self.mem_read_debugging(addr & 0b00100000_00000111) //addr % 0x2000
            },
            ROM ..= ROM_MIRRORS_END => {
                self.read_rom(addr)
            },
            _ => 0
        }
    }
}

impl Ppu {
    pub fn read_status_debugging(&self ) -> u8 {
        self.status.bits()
    }

    pub fn read_data_debugging(&self) -> u8 {
        let addr = self.address.value();

        match addr {
            0 ..= 0x1FFF => {
                self.buffer
            },
            0x2000 ..= 0x2FFF => {
                self.buffer
            },
            /* https://www.nesdev.org/wiki/PPU_palettes */
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                self.palette_table[(addr - 0x3f10) as usize]
            },
            0x3F00 ..= 0x3FFF => {
                self.palette_table[(addr - 0x3f00) as usize]
            },
            _ => panic!("Invalid PPU address: {:#X}", addr),
        }
    }
}

impl Joypad {
    pub fn read_debugging(&self) -> u8 {
        self.buffer
    }
}