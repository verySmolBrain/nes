use crate::emulator::{ bus::Bus, memory::Mem, rom::Rom };
use bitflags::bitflags;

/*  _______________ $10000  _______________
   | PRG-ROM       |       |               |
   | Upper Bank    |       |               |
   |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
   | PRG-ROM       |       |               |
   | Lower Bank    |       |               |
   |_______________| $8000 |_______________|
   | SRAM          |       | SRAM          |
   |_______________| $6000 |_______________|
   | Expansion ROM |       | Expansion ROM |
   |_______________| $4020 |_______________|
   | I/O Registers |       |               |
   |_ _ _ _ _ _ _ _| $4000 |               |
   | Mirrors       |       | I/O Registers |
   | $2000-$2007   |       |               |
   |_ _ _ _ _ _ _ _| $2008 |               |
   | I/O Registers |       |               |
   |_______________| $2000 |_______________|
   | Mirrors       |       |               |
   | $0000-$07FF   |       |               |
   |_ _ _ _ _ _ _ _| $0800 |               |
   | RAM           |       | RAM           |
   |_ _ _ _ _ _ _ _| $0200 |               |
   | Stack         |       |               |
   |_ _ _ _ _ _ _ _| $0100 |               |
   | Zero Page     |       |               |
   |_______________| $0000 |_______________|
*/

const RESET_VECTOR: usize = 0xFFFC;
const STACK_RESET: u8 = 0xfd; // Push = store first then decrement. So 8 bit off for initial.

/*
7  bit  0
---- ----
NVss DIZC
|||| ||||
|||| |||+- Carry
|||| ||+-- Zero
|||| |+--- Interrupt Disable
|||| +---- Decimal
||++------ No Cpu effect, see: the B flag
|+-------- Overflow
+--------- Negative
 */

bitflags! {
    pub struct Status: u8 {
        const NEGATIVE = 0b1000_0000;
        const OVERFLOW = 0b0100_0000;
        const BREAKONE = 0b0010_0000;
        const BREAKTWO = 0b0001_0000;
        const DECIMAL  = 0b0000_1000;
        const INTERDIS = 0b0000_0100;
        const ZERO     = 0b0000_0010;
        const CARRY    = 0b0000_0001;
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::BREAKONE | Status::INTERDIS
    }
}

pub struct Cpu {
    pub accumulator: u8, 
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub status: Status,
    pub program_counter: u16,
    pub bus: Bus
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Cpu {
            accumulator: 0, // accumulator
            register_x: 0,
            register_y: 0,
            stack_pointer: STACK_RESET,
            status: Default::default(), 
            program_counter: 0,
            bus
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status = Default::default();

        self.program_counter = self.mem_read_u16(RESET_VECTOR as u16);
    }

    pub fn load_cartridge(&mut self, program: Vec<u8>) -> Result<(), String> {
        let cartridge = Rom::new(program)?;
        let new_bus = Bus::new(cartridge);

        self.bus = new_bus;
        self.reset();

        Ok(())
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Cpu),
    {
        loop {
            callback(self);
            if !self.step() {
                return // Change later to check for flag instead of interrupt
            }
        }
    }
}