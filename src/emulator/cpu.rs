use crate::emulator::{ bus::Bus, memory::Mem, rom::Rom, ppu::Ppu };
use crate::emulator::interrupts::Interrupt;
use bitflags::bitflags;

const RESET_VECTOR: usize = 0xFFFC;
const STACK_RESET: u8 = 0xfd; // Push = store first then decrement. So 8 bit off for initial.

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
    pub program_counter: u16,

    pub status: Status,
    pub cycles: usize,
    
    pub bus: Bus,
    
    pub interrupt: Option<Interrupt>,
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
            bus,
            cycles: 0,
            interrupt: None,
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status = Default::default();
        self.cycles = 0;

        self.cycles += 7; // 7 cycles for reset
        self.bus.tick(7);

        self.program_counter = self.mem_read_u16(RESET_VECTOR as u16);
    }

    pub fn load_cartridge(&mut self, program: Vec<u8>) -> Result<(), String> {
        let cartridge = Rom::new(program)?;
        let new_bus = Bus::new(cartridge);

        self.bus = new_bus;
        self.reset();

        Ok(())
    }

    pub fn ppu_ready(&mut self) -> Option<&Ppu> {
        if self.interrupt.is_some() {
            self.return_from_interrupt();
            self.bus.ppu.handled_interrupt();
            Some(&self.bus.ppu)
        } else {
            None
        }
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