use crate::emulator::cpu::Cpu;
use crate::emulator::memory::Stack;
use crate::emulator::memory::Mem;
use crate::emulator::cpu::Status;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum INTERRUPTS {
    NMI,
    IRQ,
    FRMFIN,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Interrupt {
    pub interrupt: INTERRUPTS,
    pub addr: u16,
    pub cycles: usize,
}

impl Interrupt {
    pub fn new_nmi() -> Self {
        Interrupt {
            interrupt: INTERRUPTS::NMI,
            addr: 0xFFFA,
            cycles: 2,
        }
    }

    pub fn new_irq() -> Self {
        Interrupt {
            interrupt: INTERRUPTS::IRQ,
            addr: 0xFFFE,
            cycles: 2,
        }
    }

    pub fn new_frmfin() -> Self {
        Interrupt {
            interrupt: INTERRUPTS::FRMFIN,
            addr: 0xFFFC,
            cycles: 7,
        }
    }
}

impl Cpu {
    pub fn interrupt(&mut self, interrupt: Interrupt) {
        self.stack_push_u16(self.program_counter);

        let mut p = self.status.clone();
        p.insert(Status::BREAKONE);
        p.insert(Status::BREAKTWO);
        self.stack_push_u8(p.bits());

        self.status.insert(Status::INTERDIS);

        self.cycles += 2;
        self.bus.tick(2);

        self.program_counter = self.mem_read_u16(interrupt.addr);
        self.interrupt = Some(interrupt);
    }

    pub fn return_from_interrupt(&mut self) {
        self.interrupt = None;
        // self.status = Status::from_bits_truncate(self.stack_pop_u8());
        // self.status.remove(Status::BREAKTWO);
        // self.status.insert(Status::BREAKONE);

        // self.program_counter = self.stack_pop_u16();
        // self.interrupt = None;
    }
}