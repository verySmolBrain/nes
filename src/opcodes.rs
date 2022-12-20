use crate::cpu::AddressingMode:: { 
    Absolute, Absolute_X, Absolute_Y, Immediate, 
    Indirect_X, Indirect_Y, NoneAddressing, 
    ZeroPage, ZeroPage_X, ZeroPage_Y };
use crate::cpu::AddressingMode;
use phf::phf_map;
use lazy_static::lazy_static;

pub struct OPCode {
    pub opcode: u8,
    pub name: &'static str,
    pub bytes: u16,
    pub cycles: u8,
    pub mode: AddressingMode,
}

lazy_static! {
    pub static ref OPCODES: phf::Map<u8, OPCode> = phf_map! {
        0x00_u8 => OPCode { opcode: 0x00, name: "BRK", bytes: 1, cycles: 7, mode: NoneAddressing },
        0xaa_u8 => OPCode { opcode: 0xaa, name: "TAX", bytes: 1, cycles: 2, mode: NoneAddressing },
        0xe8_u8 => OPCode { opcode: 0xe8, name: "INX", bytes: 1, cycles: 2, mode: NoneAddressing },

        0xa9_u8 => OPCode { opcode: 0xa9, name: "LDA", bytes: 2, cycles: 2, mode: Immediate },
        0xa5_u8 => OPCode { opcode: 0xa5, name: "LDA", bytes: 2, cycles: 3, mode: ZeroPage },
        0xb5_u8 => OPCode { opcode: 0xb5, name: "LDA", bytes: 2, cycles: 4, mode: ZeroPage_X },
        0xad_u8 => OPCode { opcode: 0xad, name: "LDA", bytes: 3, cycles: 4, mode: Absolute },
        0xbd_u8 => OPCode { opcode: 0xbd, name: "LDA", bytes: 3, cycles: 4, mode: Absolute_X },
        0xb9_u8 => OPCode { opcode: 0xb9, name: "LDA", bytes: 3, cycles: 4, mode: Absolute_Y },
        0xa1_u8 => OPCode { opcode: 0xa1, name: "LDA", bytes: 2, cycles: 6, mode: Indirect_X },
        0xb1_u8 => OPCode { opcode: 0xb1, name: "LDA", bytes: 2, cycles: 5, mode: Indirect_Y },

        0x85_u8 => OPCode { opcode: 0x85, name: "STA", bytes: 2, cycles: 3, mode: ZeroPage },
        0x95_u8 => OPCode { opcode: 0x95, name: "STA", bytes: 2, cycles: 4, mode: ZeroPage_X },
        0x8d_u8 => OPCode { opcode: 0x8d, name: "STA", bytes: 3, cycles: 4, mode: Absolute },
        0x9d_u8 => OPCode { opcode: 0x9d, name: "STA", bytes: 3, cycles: 5, mode: Absolute_X },
        0x99_u8 => OPCode { opcode: 0x99, name: "STA", bytes: 3, cycles: 5, mode: Absolute_Y },
        0x81_u8 => OPCode { opcode: 0x81, name: "STA", bytes: 2, cycles: 6, mode: Indirect_X },
        0x91_u8 => OPCode { opcode: 0x91, name: "STA", bytes: 2, cycles: 6, mode: Indirect_Y },
    };
}
