

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

// impl Rom {
//     pub fn new(cartridge: Vec<u8>) -> Result<Rom, String> {
//         if &cartridge[0..4] != NES_HEADER {
//             return Err("Invalid NES Header".to_string())
//         }

//         let prg_rom_size = cartridge[4] as usize * PRG_ROM_PAGE_SIZE;
//         let chr_rom_size = cartridge[5] as usize * CHR_ROM_PAGE_SIZE;

//         let flag_6 = cartridge[6];
//         let flag_7 = cartridge[7];
//     }
// }