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

    /* Combined */
    AAC_U,
    AAX_U,
    ARR_U,
    ASR_U,
    ATX_U,
    AXA_U,
    AXS_U,

    /* RMW (read-modify-write) */
    DCP_U,
    ISC_U,
    LAR_U,
    LAX_U,
    RLA_U,
    RRA_U,
    SLO_U,
    SRE_U,
    SXA_U,
    SYA_U,
    XAS_U,

    /* Do Nothing */
    DOP_U,
    KIL_U,
    NOP_U,

    /* ඞ */
    XAA_U // XAA is amogus and shouldnt really be used
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
        Unofficial Instructions (Combined)
        =======================
    */

    0x0b_u8 => OPCode { code: Code::AAC_U, bytes: 2, cycles: 2, mode: Immediate },
    0x2b_u8 => OPCode { code: Code::AAC_U, bytes: 2, cycles: 2, mode: Immediate },

    0x87_u8 => OPCode { code: Code::AAX_U, bytes: 2, cycles: 3, mode: ZeroPage },
    0x97_u8 => OPCode { code: Code::AAX_U, bytes: 2, cycles: 4, mode: ZeroPage_Y },
    0x83_u8 => OPCode { code: Code::AAX_U, bytes: 2, cycles: 6, mode: Indirect_X },
    0x8f_u8 => OPCode { code: Code::AAX_U, bytes: 3, cycles: 4, mode: Absolute },

    0x6b_u8 => OPCode { code: Code::ARR_U, bytes: 2, cycles: 2, mode: Immediate },

    0x4b_u8 => OPCode { code: Code::ASR_U, bytes: 2, cycles: 2, mode: Immediate },

    0xab_u8 => OPCode { code: Code::ATX_U, bytes: 2, cycles: 2, mode: Immediate },

    0x9f_u8 => OPCode { code: Code::AXA_U, bytes: 3, cycles: 5, mode: Absolute_Y },
    0x93_u8 => OPCode { code: Code::AXA_U, bytes: 2, cycles: 6, mode: Indirect_Y },

    0xcb_u8 => OPCode { code: Code::AXS_U, bytes: 2, cycles: 2, mode: Immediate },

    /*
        Unofficial Instructions (RMW read-modify-write)
        =======================
    */

    0xc7_u8 => OPCode { code: Code::DCP_U, bytes: 2, cycles: 5, mode: ZeroPage },
    0xd7_u8 => OPCode { code: Code::DCP_U, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0xcf_u8 => OPCode { code: Code::DCP_U, bytes: 3, cycles: 6, mode: Absolute },
    0xdf_u8 => OPCode { code: Code::DCP_U, bytes: 3, cycles: 7, mode: Absolute_X },
    0xdb_u8 => OPCode { code: Code::DCP_U, bytes: 3, cycles: 7, mode: Absolute_Y },
    0xc3_u8 => OPCode { code: Code::DCP_U, bytes: 2, cycles: 8, mode: Indirect_X },
    0xd3_u8 => OPCode { code: Code::DCP_U, bytes: 2, cycles: 8, mode: Indirect_Y },

    0xe7_u8 => OPCode { code: Code::ISC_U, bytes: 2, cycles: 5, mode: ZeroPage },
    0xf7_u8 => OPCode { code: Code::ISC_U, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0xef_u8 => OPCode { code: Code::ISC_U, bytes: 3, cycles: 6, mode: Absolute },
    0xff_u8 => OPCode { code: Code::ISC_U, bytes: 3, cycles: 7, mode: Absolute_X },
    0xfb_u8 => OPCode { code: Code::ISC_U, bytes: 3, cycles: 7, mode: Absolute_Y },
    0xe3_u8 => OPCode { code: Code::ISC_U, bytes: 2, cycles: 8, mode: Indirect_X },
    0xf3_u8 => OPCode { code: Code::ISC_U, bytes: 2, cycles: 8, mode: Indirect_Y },

    0xbb_u8 => OPCode { code: Code::LAR_U, bytes: 3, cycles: 4, mode: Absolute_Y },

    0xa7_u8 => OPCode { code: Code::LAX_U, bytes: 2, cycles: 3, mode: ZeroPage },
    0xb7_u8 => OPCode { code: Code::LAX_U, bytes: 2, cycles: 4, mode: ZeroPage_Y },
    0xaf_u8 => OPCode { code: Code::LAX_U, bytes: 3, cycles: 4, mode: Absolute },
    0xbf_u8 => OPCode { code: Code::LAX_U, bytes: 3, cycles: 4, mode: Absolute_Y },
    0xa3_u8 => OPCode { code: Code::LAX_U, bytes: 2, cycles: 6, mode: Indirect_X },
    0xb3_u8 => OPCode { code: Code::LAX_U, bytes: 2, cycles: 5, mode: Indirect_Y },

    0x27_u8 => OPCode { code: Code::RLA_U, bytes: 2, cycles: 5, mode: ZeroPage },
    0x37_u8 => OPCode { code: Code::RLA_U, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x2f_u8 => OPCode { code: Code::RLA_U, bytes: 3, cycles: 6, mode: Absolute },
    0x3f_u8 => OPCode { code: Code::RLA_U, bytes: 3, cycles: 7, mode: Absolute_X },
    0x3b_u8 => OPCode { code: Code::RLA_U, bytes: 3, cycles: 7, mode: Absolute_Y },
    0x23_u8 => OPCode { code: Code::RLA_U, bytes: 2, cycles: 8, mode: Indirect_X },
    0x33_u8 => OPCode { code: Code::RLA_U, bytes: 2, cycles: 8, mode: Indirect_Y },

    0x67_u8 => OPCode { code: Code::RRA_U, bytes: 2, cycles: 5, mode: ZeroPage },
    0x77_u8 => OPCode { code: Code::RRA_U, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x6f_u8 => OPCode { code: Code::RRA_U, bytes: 3, cycles: 6, mode: Absolute },
    0x7f_u8 => OPCode { code: Code::RRA_U, bytes: 3, cycles: 7, mode: Absolute_X },
    0x7b_u8 => OPCode { code: Code::RRA_U, bytes: 3, cycles: 7, mode: Absolute_Y },
    0x63_u8 => OPCode { code: Code::RRA_U, bytes: 2, cycles: 8, mode: Indirect_X },
    0x73_u8 => OPCode { code: Code::RRA_U, bytes: 2, cycles: 8, mode: Indirect_Y },

    0x07_u8 => OPCode { code: Code::SLO_U, bytes: 2, cycles: 5, mode: ZeroPage },
    0x17_u8 => OPCode { code: Code::SLO_U, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x0f_u8 => OPCode { code: Code::SLO_U, bytes: 3, cycles: 6, mode: Absolute },
    0x1f_u8 => OPCode { code: Code::SLO_U, bytes: 3, cycles: 7, mode: Absolute_X },
    0x1b_u8 => OPCode { code: Code::SLO_U, bytes: 3, cycles: 7, mode: Absolute_Y },
    0x03_u8 => OPCode { code: Code::SLO_U, bytes: 2, cycles: 8, mode: Indirect_X },
    0x13_u8 => OPCode { code: Code::SLO_U, bytes: 2, cycles: 8, mode: Indirect_Y },

    0x47_u8 => OPCode { code: Code::SRE_U, bytes: 2, cycles: 5, mode: ZeroPage },
    0x57_u8 => OPCode { code: Code::SRE_U, bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x4f_u8 => OPCode { code: Code::SRE_U, bytes: 3, cycles: 6, mode: Absolute },
    0x5f_u8 => OPCode { code: Code::SRE_U, bytes: 3, cycles: 7, mode: Absolute_X },
    0x5b_u8 => OPCode { code: Code::SRE_U, bytes: 3, cycles: 7, mode: Absolute_Y },
    0x43_u8 => OPCode { code: Code::SRE_U, bytes: 2, cycles: 8, mode: Indirect_X },
    0x53_u8 => OPCode { code: Code::SRE_U, bytes: 2, cycles: 8, mode: Indirect_Y },
    
    0x9e_u8 => OPCode { code: Code::SXA_U, bytes: 3, cycles: 5, mode: Absolute_Y },

    0x9c_u8 => OPCode { code: Code::SYA_U, bytes: 3, cycles: 5, mode: Absolute_X },

    0x9b_u8 => OPCode { code: Code::XAS_U, bytes: 3, cycles: 5, mode: Absolute_Y },

    /*
        Unofficial Instructions (Do Nothing)
        =======================
    */

    0x04_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 3, mode: ZeroPage },
    0x14_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x34_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x44_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 3, mode: ZeroPage },
    0x54_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x64_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 3, mode: ZeroPage },
    0x74_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x80_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 2, mode: Immediate },
    0x82_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 2, mode: Immediate },
    0x89_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 2, mode: Immediate },
    0xc2_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 2, mode: Immediate },
    0xd4_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 4, mode: ZeroPage_X },
    0xe2_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 2, mode: Immediate },
    0xf4_u8 => OPCode { code: Code::DOP_U, bytes: 2, cycles: 4, mode: ZeroPage_X },

    0x02_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0x12_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0x22_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0x32_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0x42_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0x52_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0x62_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0x72_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0x92_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0xb2_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0xd2_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },
    0xf2_u8 => OPCode { code: Code::KIL_U, bytes: 1, cycles: 0, mode: Implied },

    0x1a_u8 => OPCode { code: Code::NOP_U, bytes: 1, cycles: 2, mode: Implied },
    0x3a_u8 => OPCode { code: Code::NOP_U, bytes: 1, cycles: 2, mode: Implied },
    0x5a_u8 => OPCode { code: Code::NOP_U, bytes: 1, cycles: 2, mode: Implied },
    0x7a_u8 => OPCode { code: Code::NOP_U, bytes: 1, cycles: 2, mode: Implied },
    0xda_u8 => OPCode { code: Code::NOP_U, bytes: 1, cycles: 2, mode: Implied },
    0xfa_u8 => OPCode { code: Code::NOP_U, bytes: 1, cycles: 2, mode: Implied },

    0x0c_u8 => OPCode { code: Code::NOP_U, bytes: 3, cycles: 4, mode: Absolute },
    0x1c_u8 => OPCode { code: Code::NOP_U, bytes: 3, cycles: 4, mode: Absolute_X },
    0x3c_u8 => OPCode { code: Code::NOP_U, bytes: 3, cycles: 4, mode: Absolute_X },
    0x5c_u8 => OPCode { code: Code::NOP_U, bytes: 3, cycles: 4, mode: Absolute_X },
    0x7c_u8 => OPCode { code: Code::NOP_U, bytes: 3, cycles: 4, mode: Absolute_X },
    0xdc_u8 => OPCode { code: Code::NOP_U, bytes: 3, cycles: 4, mode: Absolute_X },
    0xfc_u8 => OPCode { code: Code::NOP_U, bytes: 3, cycles: 4, mode: Absolute_X },

    /*
        Unofficial Instructions (ඞ)
        =======================
    */

    0xeb_u8 => OPCode { code: Code::SBC, bytes: 2, cycles: 2, mode: Immediate },

    0x8b_u8 => OPCode { code: Code::XAA_U, bytes: 2, cycles: 2, mode: Immediate },
};