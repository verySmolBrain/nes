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

7  bit  0
---- ----
NVss DIZC
|||| ||||
|||| |||+- Carry
|||| ||+-- Zero
|||| |+--- Interrupt Disable
|||| +---- Decimal
||++------ No CPU effect, see: the B flag
|+-------- Overflow
+--------- Negative
*/

pub struct CPU {
    pub register_a: u8, 
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0, 
            register_x: 0,
            status: 0, 
            program_counter: 0 // Potentially change into iter
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = self.next(&program);

            match opscode {
                0xA9 => {
                    let value = self.next(&program);
                    self.lda(value);
                },
                0xE8 => self.inx(),
                0xAA => self.tax(),
                0x00 => return,
                _ => todo!()
            }
        }
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn next(&mut self, program: &Vec<u8>) -> u8 {
        let value = program[self.program_counter as usize];
        self.program_counter += 1;
        value
    }

    fn update_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
    }

    fn update_negative_flag(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }
}