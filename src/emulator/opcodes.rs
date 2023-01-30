use crate::emulator::addressing_modes::AddressingMode:: { 
    Absolute, Absolute_X, Absolute_Y, Immediate, 
    Indirect_X, Indirect_Y, JumpIndirect, Jump,
    ZeroPage, ZeroPage_X, ZeroPage_Y, Relative, Accumulator, Implied };
use crate::emulator::addressing_modes::AddressingMode;
use std::fmt::Display;
use phf::phf_map;

pub struct OPCode {
    pub code: Code,
    pub bytes: u16,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Code {
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,

    ADC,
    SBC,

    INC,
    INX,
    INY,

    DEC,
    DEX,
    DEY,

    AND,
    ORA,
    EOR,

    JMP,
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    CMP,
    CPX,
    CPY,
    BIT,

    ASL,
    LSR,
    ROL,
    ROR,

    TAX,
    TAY,
    TXA,
    TYA,

    TSX,
    TXS,

    PHA,
    PHP,
    PLA,
    PLP,

    JSR,
    RTS,
    RTI,

    CLC,
    CLD,
    CLI,
    CLV,

    SEC,
    SED,
    SEI,

    NOP,
    BRK,
}

pub static OPCODES: phf::Map<u8, OPCode> = phf_map! {
    /*
        Load and Store Instructions
        ===========================
     */

    0xa9_u8 => OPCode { code: Code::LDA, bytes: 2, cycles: 2, mode: Immediate },
    0xa5_u8 => OPCode { code: Code::LDA, bytes: 2, cycles: 3, mode: ZeroPage },
    0xb5_u8 => OPCode { code: Code::LDA, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0xad_u8 => OPCode { code: Code::LDA, bytes: 3, cycles: 4, mode: Absolute },
    0xbd_u8 => OPCode { code: Code::LDA, bytes: 3, cycles: 4, mode: Absolute_X },
    0xb9_u8 => OPCode { code: Code::LDA, bytes: 3, cycles: 4, mode: Absolute_Y },
    0xa1_u8 => OPCode { code: Code::LDA, bytes: 2, cycles: 6, mode: Indirect_X },
    0xb1_u8 => OPCode { code: Code::LDA, bytes: 2, cycles: 5, mode: Indirect_Y },

    0xa2_u8 => OPCode { code: Code::LDX, bytes: 2, cycles: 2, mode: Immediate },
    0xa6_u8 => OPCode { code: Code::LDX, bytes: 2, cycles: 3, mode: ZeroPage },
    0xb6_u8 => OPCode { code: Code::LDX, bytes: 2, cycles: 4, mode: ZeroPage_Y },
    0xae_u8 => OPCode { code: Code::LDX, bytes: 3, cycles: 4, mode: Absolute },
    0xbe_u8 => OPCode { code: Code::LDX, bytes: 3, cycles: 4, mode: Absolute_Y },

    0xa0_u8 => OPCode { code: Code::LDY, bytes: 2, cycles: 2, mode: Immediate },
    0xa4_u8 => OPCode { code: Code::LDY, bytes: 2, cycles: 3, mode: ZeroPage },
    0xb4_u8 => OPCode { code: Code::LDY, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0xac_u8 => OPCode { code: Code::LDY, bytes: 3, cycles: 4, mode: Absolute },
    0xbc_u8 => OPCode { code: Code::LDY, bytes: 3, cycles: 4, mode: Absolute_X },


    0x85_u8 => OPCode { code: Code::STA, bytes: 2, cycles: 3, mode: ZeroPage },
    0x95_u8 => OPCode { code: Code::STA, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x8d_u8 => OPCode { code: Code::STA, bytes: 3, cycles: 4, mode: Absolute },
    0x9d_u8 => OPCode { code: Code::STA, bytes: 3, cycles: 5, mode: Absolute_X },
    0x99_u8 => OPCode { code: Code::STA, bytes: 3, cycles: 5, mode: Absolute_Y },
    0x81_u8 => OPCode { code: Code::STA, bytes: 2, cycles: 6, mode: Indirect_X },
    0x91_u8 => OPCode { code: Code::STA, bytes: 2, cycles: 6, mode: Indirect_Y },

    0x86_u8 => OPCode { code: Code::STX, bytes: 2, cycles: 3, mode: ZeroPage },
    0x96_u8 => OPCode { code: Code::STX, bytes: 2, cycles: 4, mode: ZeroPage_Y },
    0x8e_u8 => OPCode { code: Code::STX, bytes: 3, cycles: 4, mode: Absolute },

    0x84_u8 => OPCode { code: Code::STY, bytes: 2, cycles: 3, mode: ZeroPage },
    0x94_u8 => OPCode { code: Code::STY, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x8c_u8 => OPCode { code: Code::STY, bytes: 3, cycles: 4, mode: Absolute },

    
    /*
        Arithmetic Instructions
        =======================
     */

    0x69_u8 => OPCode { code: Code::ADC, bytes: 2, cycles: 2, mode: Immediate },
    0x65_u8 => OPCode { code: Code::ADC, bytes: 2, cycles: 3, mode: ZeroPage },
    0x75_u8 => OPCode { code: Code::ADC, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x6d_u8 => OPCode { code: Code::ADC, bytes: 3, cycles: 4, mode: Absolute },
    0x7d_u8 => OPCode { code: Code::ADC, bytes: 3, cycles: 4, mode: Absolute_X },
    0x79_u8 => OPCode { code: Code::ADC, bytes: 3, cycles: 4, mode: Absolute_Y },
    0x61_u8 => OPCode { code: Code::ADC, bytes: 2, cycles: 6, mode: Indirect_X },
    0x71_u8 => OPCode { code: Code::ADC, bytes: 2, cycles: 5, mode: Indirect_Y },

    0xe9_u8 => OPCode { code: Code::SBC, bytes: 2, cycles: 2, mode: Immediate },
    0xe5_u8 => OPCode { code: Code::SBC, bytes: 2, cycles: 3, mode: ZeroPage },
    0xf5_u8 => OPCode { code: Code::SBC, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0xed_u8 => OPCode { code: Code::SBC, bytes: 3, cycles: 4, mode: Absolute },
    0xfd_u8 => OPCode { code: Code::SBC, bytes: 3, cycles: 4, mode: Absolute_X },
    0xf9_u8 => OPCode { code: Code::SBC, bytes: 3, cycles: 4, mode: Absolute_Y },
    0xe1_u8 => OPCode { code: Code::SBC, bytes: 2, cycles: 6, mode: Indirect_X },
    0xf1_u8 => OPCode { code: Code::SBC, bytes: 2, cycles: 5, mode: Indirect_Y },


    /*
        Increment and Decrement Instructions
        ====================================
     */

    0xe6_u8 => OPCode { code: Code::INC, bytes: 2, cycles: 5, mode: ZeroPage },
    0xf6_u8 => OPCode { code: Code::INC, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0xee_u8 => OPCode { code: Code::INC, bytes: 3, cycles: 6, mode: Absolute },
    0xfe_u8 => OPCode { code: Code::INC, bytes: 3, cycles: 7, mode: Absolute_X },

    0xe8_u8 => OPCode { code: Code::INX, bytes: 1, cycles: 2, mode: Implied },
    0xc8_u8 => OPCode { code: Code::INY, bytes: 1, cycles: 2, mode: Implied },


    0xc6_u8 => OPCode { code: Code::DEC, bytes: 2, cycles: 5, mode: ZeroPage },
    0xd6_u8 => OPCode { code: Code::DEC, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0xce_u8 => OPCode { code: Code::DEC, bytes: 3, cycles: 6, mode: Absolute },
    0xde_u8 => OPCode { code: Code::DEC, bytes: 3, cycles: 7, mode: Absolute_X },

    0xca_u8 => OPCode { code: Code::DEX, bytes: 1, cycles: 2, mode: Implied },
    0x88_u8 => OPCode { code: Code::DEY, bytes: 1, cycles: 2, mode: Implied },
    

    /*
        Logical Instructions
        ====================
     */

    0x29_u8 => OPCode { code: Code::AND, bytes: 2, cycles: 2, mode: Immediate },
    0x25_u8 => OPCode { code: Code::AND, bytes: 2, cycles: 3, mode: ZeroPage },
    0x35_u8 => OPCode { code: Code::AND, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x2d_u8 => OPCode { code: Code::AND, bytes: 3, cycles: 4, mode: Absolute },
    0x3d_u8 => OPCode { code: Code::AND, bytes: 3, cycles: 4, mode: Absolute_X },
    0x39_u8 => OPCode { code: Code::AND, bytes: 3, cycles: 4, mode: Absolute_Y },
    0x21_u8 => OPCode { code: Code::AND, bytes: 2, cycles: 6, mode: Indirect_X },
    0x31_u8 => OPCode { code: Code::AND, bytes: 2, cycles: 5, mode: Indirect_Y },

    0x09_u8 => OPCode { code: Code::ORA, bytes: 2, cycles: 2, mode: Immediate },
    0x05_u8 => OPCode { code: Code::ORA, bytes: 2, cycles: 3, mode: ZeroPage },
    0x15_u8 => OPCode { code: Code::ORA, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x0d_u8 => OPCode { code: Code::ORA, bytes: 3, cycles: 4, mode: Absolute },
    0x1d_u8 => OPCode { code: Code::ORA, bytes: 3, cycles: 4, mode: Absolute_X },
    0x19_u8 => OPCode { code: Code::ORA, bytes: 3, cycles: 4, mode: Absolute_Y },
    0x01_u8 => OPCode { code: Code::ORA, bytes: 2, cycles: 6, mode: Indirect_X },
    0x11_u8 => OPCode { code: Code::ORA, bytes: 2, cycles: 5, mode: Indirect_Y },

    0x49_u8 => OPCode { code: Code::EOR, bytes: 2, cycles: 2, mode: Immediate },
    0x45_u8 => OPCode { code: Code::EOR, bytes: 2, cycles: 3, mode: ZeroPage },
    0x55_u8 => OPCode { code: Code::EOR, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x4d_u8 => OPCode { code: Code::EOR, bytes: 3, cycles: 4, mode: Absolute },
    0x5d_u8 => OPCode { code: Code::EOR, bytes: 3, cycles: 4, mode: Absolute_X },
    0x59_u8 => OPCode { code: Code::EOR, bytes: 3, cycles: 4, mode: Absolute_Y },
    0x41_u8 => OPCode { code: Code::EOR, bytes: 2, cycles: 6, mode: Indirect_X },
    0x51_u8 => OPCode { code: Code::EOR, bytes: 2, cycles: 5, mode: Indirect_Y },

    
    /* 
        Jump, Branch, Compare, and Test Bits
        ====================================
    */

    0x4c_u8 => OPCode { code: Code::JMP, bytes: 3, cycles: 3, mode: Jump },
    0x6c_u8 => OPCode { code: Code::JMP, bytes: 3, cycles: 5, mode: JumpIndirect }, 

    0x90_u8 => OPCode { code: Code::BCC, bytes: 2, cycles: 2, mode: Relative },
    0xb0_u8 => OPCode { code: Code::BCS, bytes: 2, cycles: 2, mode: Relative },
    0xf0_u8 => OPCode { code: Code::BEQ, bytes: 2, cycles: 2, mode: Relative },
    0x30_u8 => OPCode { code: Code::BMI, bytes: 2, cycles: 2, mode: Relative },
    0xd0_u8 => OPCode { code: Code::BNE, bytes: 2, cycles: 2, mode: Relative },
    0x10_u8 => OPCode { code: Code::BPL, bytes: 2, cycles: 2, mode: Relative },
    0x50_u8 => OPCode { code: Code::BVC, bytes: 2, cycles: 2, mode: Relative },
    0x70_u8 => OPCode { code: Code::BVS, bytes: 2, cycles: 2, mode: Relative },

    0xc9_u8 => OPCode { code: Code::CMP, bytes: 2, cycles: 2, mode: Immediate },
    0xc5_u8 => OPCode { code: Code::CMP, bytes: 2, cycles: 3, mode: ZeroPage },
    0xd5_u8 => OPCode { code: Code::CMP, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0xcd_u8 => OPCode { code: Code::CMP, bytes: 3, cycles: 4, mode: Absolute },
    0xdd_u8 => OPCode { code: Code::CMP, bytes: 3, cycles: 4, mode: Absolute_X },
    0xd9_u8 => OPCode { code: Code::CMP, bytes: 3, cycles: 4, mode: Absolute_Y },
    0xc1_u8 => OPCode { code: Code::CMP, bytes: 2, cycles: 6, mode: Indirect_X },
    0xd1_u8 => OPCode { code: Code::CMP, bytes: 2, cycles: 5, mode: Indirect_Y },

    0xe0_u8 => OPCode { code: Code::CPX, bytes: 2, cycles: 2, mode: Immediate },
    0xe4_u8 => OPCode { code: Code::CPX, bytes: 2, cycles: 3, mode: ZeroPage },
    0xec_u8 => OPCode { code: Code::CPX, bytes: 3, cycles: 4, mode: Absolute },

    0xc0_u8 => OPCode { code: Code::CPY, bytes: 2, cycles: 2, mode: Immediate },
    0xc4_u8 => OPCode { code: Code::CPY, bytes: 2, cycles: 3, mode: ZeroPage },
    0xcc_u8 => OPCode { code: Code::CPY, bytes: 3, cycles: 4, mode: Absolute },

    0x24_u8 => OPCode { code: Code::BIT, bytes: 2, cycles: 3, mode: ZeroPage },
    0x2c_u8 => OPCode { code: Code::BIT, bytes: 3, cycles: 4, mode: Absolute },


    /*
        Shift and Rotate Instructions
        =============================
     */

    0x0a_u8 => OPCode { code: Code::ASL, bytes: 1, cycles: 2, mode: Accumulator },
    0x06_u8 => OPCode { code: Code::ASL, bytes: 2, cycles: 5, mode: ZeroPage },
    0x16_u8 => OPCode { code: Code::ASL, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x0e_u8 => OPCode { code: Code::ASL, bytes: 3, cycles: 6, mode: Absolute },
    0x1e_u8 => OPCode { code: Code::ASL, bytes: 3, cycles: 7, mode: Absolute_X },

    0x4a_u8 => OPCode { code: Code::LSR, bytes: 1, cycles: 2, mode: Accumulator },
    0x46_u8 => OPCode { code: Code::LSR, bytes: 2, cycles: 5, mode: ZeroPage },
    0x56_u8 => OPCode { code: Code::LSR, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x4e_u8 => OPCode { code: Code::LSR, bytes: 3, cycles: 6, mode: Absolute },
    0x5e_u8 => OPCode { code: Code::LSR, bytes: 3, cycles: 7, mode: Absolute_X },

    0x2a_u8 => OPCode { code: Code::ROL, bytes: 1, cycles: 2, mode: Accumulator },
    0x26_u8 => OPCode { code: Code::ROL, bytes: 2, cycles: 5, mode: ZeroPage },
    0x36_u8 => OPCode { code: Code::ROL, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x2e_u8 => OPCode { code: Code::ROL, bytes: 3, cycles: 6, mode: Absolute },
    0x3e_u8 => OPCode { code: Code::ROL, bytes: 3, cycles: 7, mode: Absolute_X },

    0x6a_u8 => OPCode { code: Code::ROR, bytes: 1, cycles: 2, mode: Accumulator },
    0x66_u8 => OPCode { code: Code::ROR, bytes: 2, cycles: 5, mode: ZeroPage },
    0x76_u8 => OPCode { code: Code::ROR, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x6e_u8 => OPCode { code: Code::ROR, bytes: 3, cycles: 6, mode: Absolute },
    0x7e_u8 => OPCode { code: Code::ROR, bytes: 3, cycles: 7, mode: Absolute_X },


    /*
        Transfer Instructions
        =====================
     */
    
    0xaa_u8 => OPCode { code: Code::TAX, bytes: 1, cycles: 2, mode: Implied },
    0xa8_u8 => OPCode { code: Code::TAY, bytes: 1, cycles: 2, mode: Implied },
    0x8a_u8 => OPCode { code: Code::TXA, bytes: 1, cycles: 2, mode: Implied },
    0x98_u8 => OPCode { code: Code::TYA, bytes: 1, cycles: 2, mode: Implied },


    /*
        Stack Instructions
        ==================
     */

    0xba_u8 => OPCode { code: Code::TSX, bytes: 1, cycles: 2, mode: Implied },
    0x9a_u8 => OPCode { code: Code::TXS, bytes: 1, cycles: 2, mode: Implied },

    0x48_u8 => OPCode { code: Code::PHA, bytes: 1, cycles: 3, mode: Implied },
    0x08_u8 => OPCode { code: Code::PHP, bytes: 1, cycles: 3, mode: Implied },
    0x68_u8 => OPCode { code: Code::PLA, bytes: 1, cycles: 4, mode: Implied },
    0x28_u8 => OPCode { code: Code::PLP, bytes: 1, cycles: 4, mode: Implied },


    /*
        Subroutine Instructions
        =======================
     */

    0x20_u8 => OPCode { code: Code::JSR, bytes: 3, cycles: 6, mode: Jump },
    0x60_u8 => OPCode { code: Code::RTS, bytes: 1, cycles: 6, mode: Implied },
    0x40_u8 => OPCode { code: Code::RTI, bytes: 1, cycles: 6, mode: Implied },
    

    /*
        Set and Reset (Clear) Instructions
        ==================================
     */
    
    0x18_u8 => OPCode { code: Code::CLC, bytes: 1, cycles: 2, mode: Implied },
    0xd8_u8 => OPCode { code: Code::CLD, bytes: 1, cycles: 2, mode: Implied },
    0x58_u8 => OPCode { code: Code::CLI, bytes: 1, cycles: 2, mode: Implied },
    0xb8_u8 => OPCode { code: Code::CLV, bytes: 1, cycles: 2, mode: Implied },

    0x38_u8 => OPCode { code: Code::SEC, bytes: 1, cycles: 2, mode: Implied },
    0xf8_u8 => OPCode { code: Code::SED, bytes: 1, cycles: 2, mode: Implied },
    0x78_u8 => OPCode { code: Code::SEI, bytes: 1, cycles: 2, mode: Implied },
    

    /*
        Other Instructions
        ==================
     */

    0xea_u8 => OPCode { code: Code::NOP, bytes: 1, cycles: 2, mode: Implied },
    0x00_u8 => OPCode { code: Code::BRK, bytes: 1, cycles: 7, mode: Implied },

    /*
        Unofficial Instructions
        =======================
     */
};