use crate::emulator::rom::Mirroring;
use bitflags::bitflags;

bitflags! {
    pub struct Controller: u8 {
        const NMI_INTERRUPT  = 0b1000_0000;
        const MASTER_SLAVE   = 0b0100_0000;
        const SPRITE_SIZE    = 0b0010_0000;
        const BACKGROUND     = 0b0001_0000;
        const SPRITES_ADDR   = 0b0000_1000;
        const VRAM_INCREMENT = 0b0000_0100;
        const NAME_TABLE_0   = 0b0000_0010;
        const NAME_TABLE_1   = 0b0000_0001;
    }
}

bitflags! {
    pub struct Mask: u8 {
        const EMPHASIZE_BLUE  = 0b1000_0000;
        const EMPHASIZE_GREEN = 0b0100_0000;
        const EMPHASIZE_RED   = 0b0010_0000;
        const SPRITE_ENABLE   = 0b0001_0000;
        const BG_ENABLE       = 0b0000_1000;
        const SPRITE_LEFTMOST = 0b0000_0100;
        const BG_LEFTMOST     = 0b0000_0010;
        const GREYSCALE       = 0b0000_0001;
    }
}

bitflags! {
    pub struct Status: u8 {
        const VBLANK_STARTED  = 0b1000_0000;
        const SPRITE_0        = 0b0100_0000;
        const SPRITE_OVERFLOW = 0b0010_0000;
    }
}

pub struct Scroll {
    pub x: u8,
    pub y: u8,
    pub latch: bool,
}

pub struct Address {
    pub h: u8,
    pub l: u8,
    pub latch: bool,
}

impl Address {
    pub fn value(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l]) & 0x3FFF
    }

    pub fn next(&mut self, controller: Controller) {
        let inc = if controller.contains(Controller::VRAM_INCREMENT) { 
            0x20 
        } else { 1 };

        if self.l > self.l.wrapping_add(inc) {
            self.h = self.h.wrapping_add(1);
        }
        self.l = self.l.wrapping_add(inc);
    }
}

pub struct Ppu {
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub buffer: u8,

    pub controller: Controller,
    pub mask: Mask,
    pub status: Status,
    pub oam_addr: u8,
    pub oam_data: [u8; 256],
    pub scroll: Scroll,
    pub address: Address,
}

impl Ppu {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Ppu {
            chr_rom,
            mirroring,
            palette_table: [0; 32],
            vram: [0; 0x800],
            buffer: 0,

            controller: Controller::empty(),
            mask: Mask::empty(),
            status: Status::empty(),
            oam_addr: 0,
            oam_data: [0; 256],
            scroll: Scroll   { x: 0, y: 0, latch: false },
            address: Address { h: 0, l: 0, latch: false },
        }
    }

    pub fn read_status(&mut self) -> u8 {
        let status = self.status.bits();

        self.status.remove(Status::VBLANK_STARTED);
        self.scroll.latch = false;
        self.address.latch = false;

        status
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.address.value();
        let value: u8;

        match addr {
            0 ..= 0x1FFF => {
                value = self.buffer;
                self.buffer = self.chr_rom[addr as usize];
            },
            0x2000 ..= 0x2FFF => {
                value = self.buffer;
                self.buffer = self.vram[self.mirror_vram(addr) as usize];
            },
            /* https://www.nesdev.org/wiki/PPU_palettes */
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                value = self.palette_table[(addr - 0x3f10) as usize];
            },
            0x3F00 ..= 0x3FFF => {
                value = self.palette_table[(addr - 0x3f00) as usize];
            },
            _ => panic!("Invalid PPU address: {:#X}", addr),
        }

        self.address.next(self.controller);
        value
    }

    pub fn write_controller(&mut self, value: u8) {
        if self.controller.contains(Controller::NMI_INTERRUPT) {
            todo!("NMI Interrupt");
        }
        self.controller = Controller::from_bits_truncate(value);
    }

    pub fn write_mask(&mut self, value: u8) {
        self.mask = Mask::from_bits_truncate(value);
    }

    pub fn write_oam_addr(&mut self, value: u8) {
        self.oam_addr = value;
    }

    pub fn write_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn write_scroll(&mut self, value: u8) {
        if self.scroll.latch {
            self.scroll.y = value;
        } else {
            self.scroll.x = value;
        }
        self.scroll.latch = !self.scroll.latch;
    }   

    pub fn write_address(&mut self, value: u8) {
        if self.address.latch { 
            self.address.l = value;
        } else { // Write BE so MSB first
            self.address.h = value;
        }
        self.address.latch = !self.address.latch;
    }

    pub fn write_data(&mut self, value: u8) {
        let addr = self.address.value();

        match addr {
            0x2000 ..= 0x2FFF => {
                self.vram[self.mirror_vram(addr) as usize] = value;
            },
            /* https://www.nesdev.org/wiki/PPU_palettes */
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                self.palette_table[(addr - 0x3f10) as usize] = value;
            },
            0x3F00 ..= 0x3FFF => {
                self.palette_table[(addr - 0x3f00) as usize] = value;
            },
            _ => panic!("Invalid PPU address: {:#X}", addr),
        }

        self.address.next(self.controller);
    }

    pub fn write_oam_dma(&mut self, oam: &[u8; 256]) {
        oam.iter().for_each(|&byte|
            self.write_oam_data(byte)
        );
    }

    /*
        Grid Idx
        +---+---+
        | 0 | 1 |
        +---+---+
        | 2 | 3 |
        +---+---+
     */
    pub fn mirror_vram(&self, addr: u16) -> u16 {
        let vram_idx = addr - 0x2000;
        let grid_idx = vram_idx / 0x400;
        match self.mirroring {
            Mirroring::HORIZONTAL => {
                match grid_idx {
                    0 => vram_idx,
                    1 | 2 => vram_idx - 0x400,
                    3 => vram_idx - 0x800,
                    _ => panic!("Invalid VRAM index: {:#X}", vram_idx),
                }
            },
            Mirroring::VERTICAL => {
                match grid_idx {
                    0 | 1 => vram_idx,
                    2 | 3 => vram_idx - 0x800,
                    _ => panic!("Invalid VRAM index: {:#X}", vram_idx),
                }
            },
            Mirroring::FOURSCREEN => panic!("Four screen mirroring not supported"),
        }
    }
}