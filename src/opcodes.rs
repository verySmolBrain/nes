use crate::cpu::AddressingMode:: { 
    Absolute, Absolute_X, Absolute_Y, Immediate, 
    Indirect_X, Indirect_Y, NoneAddressing, 
    ZeroPage, ZeroPage_X, ZeroPage_Y, Relative, Accumulator, Implied };
use crate::cpu::AddressingMode;
use phf::phf_map;

pub struct OPCode {
    pub name: &'static str,
    pub bytes: u16,
    pub cycles: u8,
    pub mode: AddressingMode,
}

pub static OPCODES: phf::Map<u8, OPCode> = phf_map! {
    0x00_u8 => OPCode { name: "BRK", bytes: 1, cycles: 7, mode: Implied },
    0xaa_u8 => OPCode { name: "TAX", bytes: 1, cycles: 2, mode: Implied },
    0xe8_u8 => OPCode { name: "INX", bytes: 1, cycles: 2, mode: Implied },

    0xa9_u8 => OPCode { name: "LDA", bytes: 2, cycles: 2, mode: Immediate },
    0xa5_u8 => OPCode { name: "LDA", bytes: 2, cycles: 3, mode: ZeroPage },
    0xb5_u8 => OPCode { name: "LDA", bytes: 2, cycles: 4, mode: ZeroPage_X },
    0xad_u8 => OPCode { name: "LDA", bytes: 3, cycles: 4, mode: Absolute },
    0xbd_u8 => OPCode { name: "LDA", bytes: 3, cycles: 4, mode: Absolute_X },
    0xb9_u8 => OPCode { name: "LDA", bytes: 3, cycles: 4, mode: Absolute_Y },
    0xa1_u8 => OPCode { name: "LDA", bytes: 2, cycles: 6, mode: Indirect_X },
    0xb1_u8 => OPCode { name: "LDA", bytes: 2, cycles: 5, mode: Indirect_Y },

    0x85_u8 => OPCode { name: "STA", bytes: 2, cycles: 3, mode: ZeroPage },
    0x95_u8 => OPCode { name: "STA", bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x8d_u8 => OPCode { name: "STA", bytes: 3, cycles: 4, mode: Absolute },
    0x9d_u8 => OPCode { name: "STA", bytes: 3, cycles: 5, mode: Absolute_X },
    0x99_u8 => OPCode { name: "STA", bytes: 3, cycles: 5, mode: Absolute_Y },
    0x81_u8 => OPCode { name: "STA", bytes: 2, cycles: 6, mode: Indirect_X },
    0x91_u8 => OPCode { name: "STA", bytes: 2, cycles: 6, mode: Indirect_Y },

    0x84_u8 => OPCode { name: "STY", bytes: 2, cycles: 3, mode: ZeroPage },
    0x94_u8 => OPCode { name: "STY", bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x8c_u8 => OPCode { name: "STY", bytes: 3, cycles: 4, mode: Absolute },

    0x86_u8 => OPCode { name: "STX", bytes: 2, cycles: 3, mode: ZeroPage },
    0x96_u8 => OPCode { name: "STX", bytes: 2, cycles: 4, mode: ZeroPage_Y },
    0x8e_u8 => OPCode { name: "STX", bytes: 3, cycles: 4, mode: Absolute },

    0xc9_u8 => OPCode { name: "CMP", bytes: 2, cycles: 2, mode: Immediate },
    0xc5_u8 => OPCode { name: "CMP", bytes: 2, cycles: 3, mode: ZeroPage },
    0xd5_u8 => OPCode { name: "CMP", bytes: 2, cycles: 4, mode: ZeroPage_X },
    0xcd_u8 => OPCode { name: "CMP", bytes: 3, cycles: 4, mode: Absolute },
    0xdd_u8 => OPCode { name: "CMP", bytes: 3, cycles: 4, mode: Absolute_X },
    0xd9_u8 => OPCode { name: "CMP", bytes: 3, cycles: 4, mode: Absolute_Y },
    0xc1_u8 => OPCode { name: "CMP", bytes: 2, cycles: 6, mode: Indirect_X },
    0xd1_u8 => OPCode { name: "CMP", bytes: 2, cycles: 5, mode: Indirect_Y },

    0xe0_u8 => OPCode { name: "CPX", bytes: 2, cycles: 2, mode: Immediate },
    0xe4_u8 => OPCode { name: "CPX", bytes: 2, cycles: 3, mode: ZeroPage },
    0xec_u8 => OPCode { name: "CPX", bytes: 3, cycles: 4, mode: Absolute },

    0xc0_u8 => OPCode { name: "CPY", bytes: 2, cycles: 2, mode: Immediate },
    0xc4_u8 => OPCode { name: "CPY", bytes: 2, cycles: 3, mode: ZeroPage },
    0xcc_u8 => OPCode { name: "CPY", bytes: 3, cycles: 4, mode: Absolute },

    0x38_u8 => OPCode { name: "SEC", bytes: 1, cycles: 2, mode: Implied },
    0xf8_u8 => OPCode { name: "SED", bytes: 1, cycles: 2, mode: Implied },
    0x78_u8 => OPCode { name: "SEI", bytes: 1, cycles: 2, mode: Implied },
    0xa8_u8 => OPCode { name: "TAY", bytes: 1, cycles: 2, mode: Implied },
    0xba_u8 => OPCode { name: "TSX", bytes: 1, cycles: 2, mode: Implied },
    0x8a_u8 => OPCode { name: "TXA", bytes: 1, cycles: 2, mode: Implied },
    0x9a_u8 => OPCode { name: "TXS", bytes: 1, cycles: 2, mode: Implied },
    0x98_u8 => OPCode { name: "TYA", bytes: 1, cycles: 2, mode: Implied },
    0x18_u8 => OPCode { name: "CLC", bytes: 1, cycles: 2, mode: Implied },
    0xd8_u8 => OPCode { name: "CLD", bytes: 1, cycles: 2, mode: Implied },
    0x58_u8 => OPCode { name: "CLI", bytes: 1, cycles: 2, mode: Implied },
    0xb8_u8 => OPCode { name: "CLV", bytes: 1, cycles: 2, mode: Implied },
    
    0xc6_u8 => OPCode { name: "DEC", bytes: 2, cycles: 5, mode: ZeroPage },
    0xd6_u8 => OPCode { name: "DEC", bytes: 2, cycles: 6, mode: ZeroPage_X },
    0xce_u8 => OPCode { name: "DEC", bytes: 3, cycles: 6, mode: Absolute },
    0xde_u8 => OPCode { name: "DEC", bytes: 3, cycles: 7, mode: Absolute_X },

    0xca_u8 => OPCode { name: "DEX", bytes: 1, cycles: 2, mode: Implied },

    0x88_u8 => OPCode { name: "DEY", bytes: 1, cycles: 2, mode: Implied },
    
    0xe6_u8 => OPCode { name: "INC", bytes: 2, cycles: 5, mode: ZeroPage },
    0xf6_u8 => OPCode { name: "INC", bytes: 2, cycles: 6, mode: ZeroPage_X },
    0xee_u8 => OPCode { name: "INC", bytes: 3, cycles: 6, mode: Absolute },
    0xfe_u8 => OPCode { name: "INC", bytes: 3, cycles: 7, mode: Absolute_X },

    0xc8_u8 => OPCode { name: "INY", bytes: 1, cycles: 2, mode: Implied },

    0xa2_u8 => OPCode { name: "LDX", bytes: 2, cycles: 2, mode: Immediate },
    0xa6_u8 => OPCode { name: "LDX", bytes: 2, cycles: 3, mode: ZeroPage },
    0xb6_u8 => OPCode { name: "LDX", bytes: 2, cycles: 4, mode: ZeroPage_Y },
    0xae_u8 => OPCode { name: "LDX", bytes: 3, cycles: 4, mode: Absolute },
    0xbe_u8 => OPCode { name: "LDX", bytes: 3, cycles: 4, mode: Absolute_Y },

    0xa0_u8 => OPCode { name: "LDY", bytes: 2, cycles: 2, mode: Immediate },
    0xa4_u8 => OPCode { name: "LDY", bytes: 2, cycles: 3, mode: ZeroPage },
    0xb4_u8 => OPCode { name: "LDY", bytes: 2, cycles: 4, mode: ZeroPage_X },
    0xac_u8 => OPCode { name: "LDY", bytes: 3, cycles: 4, mode: Absolute },
    0xbc_u8 => OPCode { name: "LDY", bytes: 3, cycles: 4, mode: Absolute_X },

    0xea_u8 => OPCode { name: "NOP", bytes: 1, cycles: 2, mode: Implied },

    0x48_u8 => OPCode { name: "PHA", bytes: 1, cycles: 3, mode: Implied },

    0x08_u8 => OPCode { name: "PHP", bytes: 1, cycles: 3, mode: Implied },

    0x68_u8 => OPCode { name: "PLA", bytes: 1, cycles: 4, mode: Implied },

    0x28_u8 => OPCode { name: "PLP", bytes: 1, cycles: 4, mode: Implied },

    0x29_u8 => OPCode { name: "AND", bytes: 2, cycles: 2, mode: Immediate },
    0x25_u8 => OPCode { name: "AND", bytes: 2, cycles: 3, mode: ZeroPage },
    0x35_u8 => OPCode { name: "AND", bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x2d_u8 => OPCode { name: "AND", bytes: 3, cycles: 4, mode: Absolute },
    0x3d_u8 => OPCode { name: "AND", bytes: 3, cycles: 4, mode: Absolute_X },
    0x39_u8 => OPCode { name: "AND", bytes: 3, cycles: 4, mode: Absolute_Y },
    0x21_u8 => OPCode { name: "AND", bytes: 2, cycles: 6, mode: Indirect_X },
    0x31_u8 => OPCode { name: "AND", bytes: 2, cycles: 5, mode: Indirect_Y },

    0x09_u8 => OPCode { name: "ORA", bytes: 2, cycles: 2, mode: Immediate },
    0x05_u8 => OPCode { name: "ORA", bytes: 2, cycles: 3, mode: ZeroPage },
    0x15_u8 => OPCode { name: "ORA", bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x0d_u8 => OPCode { name: "ORA", bytes: 3, cycles: 4, mode: Absolute },
    0x1d_u8 => OPCode { name: "ORA", bytes: 3, cycles: 4, mode: Absolute_X },
    0x19_u8 => OPCode { name: "ORA", bytes: 3, cycles: 4, mode: Absolute_Y },
    0x01_u8 => OPCode { name: "ORA", bytes: 2, cycles: 6, mode: Indirect_X },
    0x11_u8 => OPCode { name: "ORA", bytes: 2, cycles: 5, mode: Indirect_Y },

    0x49_u8 => OPCode { name: "EOR", bytes: 2, cycles: 2, mode: Immediate },
    0x45_u8 => OPCode { name: "EOR", bytes: 2, cycles: 3, mode: ZeroPage },
    0x55_u8 => OPCode { name: "EOR", bytes: 2, cycles: 4, mode: ZeroPage_X },
    0x4d_u8 => OPCode { name: "EOR", bytes: 3, cycles: 4, mode: Absolute },
    0x5d_u8 => OPCode { name: "EOR", bytes: 3, cycles: 4, mode: Absolute_X },
    0x59_u8 => OPCode { name: "EOR", bytes: 3, cycles: 4, mode: Absolute_Y },
    0x41_u8 => OPCode { name: "EOR", bytes: 2, cycles: 6, mode: Indirect_X },
    0x51_u8 => OPCode { name: "EOR", bytes: 2, cycles: 5, mode: Indirect_Y },

    0x90_u8 => OPCode { name: "BCC", bytes: 2, cycles: 2, mode: Relative },

    0xb0_u8 => OPCode { name: "BCS", bytes: 2, cycles: 2, mode: Relative },

    0xf0_u8 => OPCode { name: "BEQ", bytes: 2, cycles: 2, mode: Relative },

    0x30_u8 => OPCode { name: "BMI", bytes: 2, cycles: 2, mode: Relative },

    0xd0_u8 => OPCode { name: "BNE", bytes: 2, cycles: 2, mode: Relative },

    0x10_u8 => OPCode { name: "BPL", bytes: 2, cycles: 2, mode: Relative },

    0x50_u8 => OPCode { name: "BVC", bytes: 2, cycles: 2, mode: Relative },

    0x70_u8 => OPCode { name: "BVS", bytes: 2, cycles: 2, mode: Relative },

    0x4c_u8 => OPCode { name: "JMP", bytes: 3, cycles: 3, mode: Absolute },
    // Indirect but due to 'bug' in 6502, it doesn't mesh well with the other addressing modes
    0x6c_u8 => OPCode { name: "JMP", bytes: 3, cycles: 5, mode: NoneAddressing }, 

    0x24_u8 => OPCode { name: "BIT", bytes: 2, cycles: 3, mode: ZeroPage },
    0x2c_u8 => OPCode { name: "BIT", bytes: 3, cycles: 4, mode: Absolute },

    0x2a_u8 => OPCode { name: "ROL", bytes: 1, cycles: 2, mode: Accumulator },
    0x26_u8 => OPCode { name: "ROL", bytes: 2, cycles: 5, mode: ZeroPage },
    0x36_u8 => OPCode { name: "ROL", bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x2e_u8 => OPCode { name: "ROL", bytes: 3, cycles: 6, mode: Absolute },
    0x3e_u8 => OPCode { name: "ROL", bytes: 3, cycles: 7, mode: Absolute_X },

    0x6a_u8 => OPCode { name: "ROR", bytes: 1, cycles: 2, mode: Accumulator },
    0x66_u8 => OPCode { name: "ROR", bytes: 2, cycles: 5, mode: ZeroPage },
    0x76_u8 => OPCode { name: "ROR", bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x6e_u8 => OPCode { name: "ROR", bytes: 3, cycles: 6, mode: Absolute },
    0x7e_u8 => OPCode { name: "ROR", bytes: 3, cycles: 7, mode: Absolute_X },

    0x0a_u8 => OPCode { name: "ASL", bytes: 1, cycles: 2, mode: Accumulator },
    0x06_u8 => OPCode { name: "ASL", bytes: 2, cycles: 5, mode: ZeroPage },
    0x16_u8 => OPCode { name: "ASL", bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x0e_u8 => OPCode { name: "ASL", bytes: 3, cycles: 6, mode: Absolute },
    0x1e_u8 => OPCode { name: "ASL", bytes: 3, cycles: 7, mode: Absolute_X },

    0x4a_u8 => OPCode { name: "LSR", bytes: 1, cycles: 2, mode: Accumulator },
    0x46_u8 => OPCode { name: "LSR", bytes: 2, cycles: 5, mode: ZeroPage },
    0x56_u8 => OPCode { name: "LSR", bytes: 2, cycles: 6, mode: ZeroPage_X },
    0x4e_u8 => OPCode { name: "LSR", bytes: 3, cycles: 6, mode: Absolute },
    0x5e_u8 => OPCode { name: "LSR", bytes: 3, cycles: 7, mode: Absolute_X },

    0x40_u8 => OPCode { name: "RTI", bytes: 1, cycles: 6, mode: Implied },

    0x60_u8 => OPCode { name: "RTS", bytes: 1, cycles: 6, mode: Implied },

    0x20_u8 => OPCode { name: "JSR", bytes: 3, cycles: 6, mode: Absolute },

    // ADC
    // SBC
};

