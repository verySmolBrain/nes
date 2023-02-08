use crate::emulator::rom::Rom;
use crate::emulator::ppu::Ppu;

const BUS_ADDRESS_SPACE: usize = 0x800;

pub struct Bus {
    pub cpu_vram: [u8; BUS_ADDRESS_SPACE],
    pub prg_rom: Vec<u8>,
    pub ppu: Ppu,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            cpu_vram: [0; BUS_ADDRESS_SPACE],
            ppu: Ppu::new(rom.chr_rom, rom.screen_mirroring),
            prg_rom: rom.prg_rom,
        }
    }

    pub fn read_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000; // Mirror in case of 16KB ROM
        }
        self.prg_rom[addr as usize]
    }

    pub fn tick(&mut self, cycles: usize) {
        self.ppu.tick(cycles * 3);
    }
}