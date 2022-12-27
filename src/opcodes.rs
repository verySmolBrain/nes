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

        0x84_u8 => OPCode { opcode: 0x84, name: "STY", bytes: 2, cycles: 3, mode: ZeroPage },
        0x94_u8 => OPCode { opcode: 0x94, name: "STY", bytes: 2, cycles: 4, mode: ZeroPage_X },
        0x8c_u8 => OPCode { opcode: 0x8c, name: "STY", bytes: 3, cycles: 4, mode: Absolute },

        0x86_u8 => OPCode { opcode: 0x86, name: "STX", bytes: 2, cycles: 3, mode: ZeroPage },
        0x96_u8 => OPCode { opcode: 0x96, name: "STX", bytes: 2, cycles: 4, mode: ZeroPage_Y },
        0x8e_u8 => OPCode { opcode: 0x8e, name: "STX", bytes: 3, cycles: 4, mode: Absolute },

        0x38_u8 => OPCode { opcode: 0x38, name: "SEC", bytes: 1, cycles: 2, mode: NoneAddressing },
        0xf8_u8 => OPCode { opcode: 0xf8, name: "SED", bytes: 1, cycles: 2, mode: NoneAddressing },
        0x78_u8 => OPCode { opcode: 0x78, name: "SEI", bytes: 1, cycles: 2, mode: NoneAddressing },
        0xa8_u8 => OPCode { opcode: 0xa8, name: "TAY", bytes: 1, cycles: 2, mode: NoneAddressing },
        0xba_u8 => OPCode { opcode: 0xba, name: "TSX", bytes: 1, cycles: 2, mode: NoneAddressing },
        0x8a_u8 => OPCode { opcode: 0x8a, name: "TXA", bytes: 1, cycles: 2, mode: NoneAddressing },
        0x9a_u8 => OPCode { opcode: 0x9a, name: "TXS", bytes: 1, cycles: 2, mode: NoneAddressing },
        0x98_u8 => OPCode { opcode: 0x98, name: "TYA", bytes: 1, cycles: 2, mode: NoneAddressing },

        // CLC
        0x18_u8 => OPCode { opcode: 0x18, name: "CLC", bytes: 1, cycles: 2, mode: NoneAddressing },
        // CLD
        0xd8_u8 => OPCode { opcode: 0xd8, name: "CLD", bytes: 1, cycles: 2, mode: NoneAddressing },
        // CLI
        0x58_u8 => OPCode { opcode: 0x58, name: "CLI", bytes: 1, cycles: 2, mode: NoneAddressing },
        // CLV
        0xb8_u8 => OPCode { opcode: 0xb8, name: "CLV", bytes: 1, cycles: 2, mode: NoneAddressing },
        // CMP
        

        // ADC
        // SBC
        // AND
        // ASL
        // BCC
        // BCS
        // BEQ
        // BIT
        // BMI
        // BNE
        // BPL
        // BVC
        // BVS
        // CPX
        // CPY
        // DEC
        // DEX
        // DEY
        // EOR
        // INC
        // INX
        // INY
        // JMP
        // JSR
        // LDX
        // LDY
        // LSR
        // NOP
        // ORA
        // PHA
        // PHP
        // PLA
        // PLP
        // ROL
        // ROR
        // RTI
        // RTS
    };
}
