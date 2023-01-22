

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    VERTICAL,
    HORIZONTAL,
    FOURSCREEN,
}

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

const NES_HEADER: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 0x4000;
const CHR_ROM_PAGE_SIZE: usize = 0x2000;

impl Rom {
    pub fn new(cartridge: Vec<u8>) -> Result<Rom, String> {
        if &cartridge[0..4] != NES_HEADER {
            return Err("Invalid NES Header".to_string())
        }

        let prg_rom_size = cartridge[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = cartridge[5] as usize * CHR_ROM_PAGE_SIZE;

        let flag_6 = cartridge[6];
        let flag_7 = cartridge[7];

        let mapper = (flag_6 >> 4) | (flag_7 & 0b1111_0000);

        let screen_mirroring = if flag_6 & 0b0000_0001 != 0 {
            Mirroring::VERTICAL
        } else if flag_6 & 0b0000_1000 != 0 {
            Mirroring::FOURSCREEN
        } else {
            Mirroring::HORIZONTAL
        };

        let trainer_exists = flag_6 & 0b0000_0100 != 0;

        let prg_rom_start = 16 + if trainer_exists { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        let nes_version = if flag_7 | 0b0000_0000 == 0 {
            1
        } else {
            2
        };

        if nes_version != 1 {
            return Err("Unsupported NES Version".to_string())
        }

        Ok(Rom {
            prg_rom: cartridge[prg_rom_start..prg_rom_start + prg_rom_size].to_vec(),
            chr_rom: cartridge[chr_rom_start..chr_rom_start + chr_rom_size].to_vec(),
            mapper,
            screen_mirroring,
        })
    }
}