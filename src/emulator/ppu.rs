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
            vram: [0; 2048],
            buffer: 0,

            controller: Controller::empty(),
            mask: Mask::empty(),
            status: Status::empty(),
            oam_addr: 0,
            oam_data: [0; 256],
            scroll: Scroll { x: 0, y: 0, latch: false },
            address: Address { value: 0, latch: false },
        }
    }

    pub fn read_status(&self) -> u8 {
        0
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.address.value;
        self.address.value += if self.controller.contains(Controller::VRAM_ADDR_INC) { 32 } else { 1 };

        match addr {
            0 ..= 0x1FFF => {
                let value = self.buffer;
                self.buffer = self.chr_rom[addr as usize];
                value
            },
            0x2000 ..= 0x3EFF => self.vram[addr as usize],
            0x3F00 ..= 0x3FFF => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("Invalid PPU address: {:#X}", addr),
        }
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

    }

    pub fn write_data(&mut self, value: u8) {

    }
}