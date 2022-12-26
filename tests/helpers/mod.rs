use nes::cpu::CPU;
use nes::cpu::Status;

pub fn check_zero_and_negative(cpu: CPU, value: u8) {
    if value == 0 {
        assert!(cpu.status.contains(Status::ZERO));
    } else {
        assert!(!cpu.status.contains(Status::ZERO));
    }
    if value & 0b1000_0000 != 0 {
        assert!(cpu.status.contains(Status::NEGATIVE));
    } else {
        assert!(!cpu.status.contains(Status::NEGATIVE));
    }
}