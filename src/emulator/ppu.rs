use crate::emulator::rom::Mirroring;
use bitflags::bitflags;

bitflags! {
    pub struct Controller: u8 {
        const NMI_INTERRUPT = 0b1000_0000;
        const MASTER_SLAVE  = 0b0100_0000;
        const SPRITE_SIZE   = 0b0010_0000;
        const BACKGROUND    = 0b0001_0000;
        const SPRITES_ADDR  = 0b0000_1000;
        const VRAM_ADDR_INC = 0b0000_0100;
        const NAME_TABLE_0  = 0b0000_0010;
        const NAME_TABLE_1  = 0b0000_0001;
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
        const V_BLANK_STARTED = 0b1000_0000;
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
    pub value: u16,
    pub latch: bool,
}

struct Ppu {
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],

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
            vram: [0; 2048],

            controller: Controller::empty(),
            mask: Mask::empty(),
            status: Status::empty(),
            oam_addr: 0,
            oam_data: [0; 256],
            scroll: Scroll { x: 0, y: 0, latch: false },
            address: Address { value: 0, latch: false },
        }
    }
}