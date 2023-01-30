use crate::emulator::cpu::Cpu;
use crate::emulator::cpu::Status;

impl Cpu {
    pub fn update_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status.insert(Status::ZERO)
        } else {
            self.status.remove(Status::ZERO)
        }
    }

    pub fn update_negative_flag(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 { 
            self.status.insert(Status::NEGATIVE)
        } else { // 6502 Integers are neither signed or unsigned. Neg depends on the most significant bit.
            self.status.remove(Status::NEGATIVE)
        }
    }

    pub fn update_carry_flag(&mut self, v1: u8, v2: u8) {
        if v1 >= v2 {
            self.status.insert(Status::CARRY)
        } else {
            self.status.remove(Status::CARRY)
        }
    }

    pub fn update_zero_and_negative_flag(&mut self, value: u8) {
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }
}