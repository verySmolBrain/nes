use crate::emulator::rom::Rom;

const BUS_ADDRESS_SPACE: usize = 0x800;

pub struct Bus {
    pub cpu_vram: [u8; BUS_ADDRESS_SPACE],
    pub rom: Rom,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            cpu_vram: [0; BUS_ADDRESS_SPACE],
            rom,
        }
    }

    pub fn read_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000; // Mirror in case of 16KB ROM
        }
        self.rom.prg_rom[addr as usize]
    }
}