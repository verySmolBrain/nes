/*
Overview of Emulator
CPU 
- 16-bit for memory addressing
CPU Memory Map
    RAM - [0x0000 … 0x2000]
    NES hardware modules: PPU, APU, GamePads - [0x2000 … 0x4020]
    Mappers - [0x4020 .. 0x6000]
    Cartridge RAM - [0x6000 .. 0x8000]
    Program ROM on Cartridge - [0x8000 … 0x10000]
CPU Registers
Program Counter is 16 bits. Others are 8 bits.
- Program Counter (PC) - holds address for next machine language instruction
- Stack Pointer - Memory space [0x0100 .. 0x1FF]. Holds the address of the top (Grows from top to bottom)
- Accumulator (A) - stores results of arithmetic, logic and memory access operations 
- Index Register X (X) - used as an offset in specific memory addressing modes. Can be used for auxiliary storage needs (Temp values etc.)
- Index Register Y (Y) - same as X 
- Processor Status (P) - represents 7 status flags that can be set or unset
Processor Status Flags
- N - Negative Flag - Set after any operation
- V - Overflow Flag
- B - Break Flag - Distinguish hardware interrupts from software interrupts
- D - Decimal Flag - Select Decimal mode 
- I - Interrupt Disable Flag - Disable CPU interrupts
- Z - Zero Flag - Set if last operation result was 0 
- C - Carry Flag - Carryover for bigger than 8-bit numbers
- 1 - Unused flag that is always set to 1
*/

pub struct CPU {
    pub register_a: u8, 
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0, 
            status: 0, 
            program_counter:0
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        todo!("Implement CPU interpreter");
    }
}