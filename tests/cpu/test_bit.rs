#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;
   
    #[test]
    fn test_0x24_bit_zero_page_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x24, 0xa1, 0x00]);
        cpu.reset();

        cpu.status.remove(Status::ZERO);

        cpu.register_a = 0b0000_1111;
        cpu.memory[0xa1] = 0b1111_0000;

        cpu.run();
        assert!(cpu.status.contains(Status::ZERO));
    }

    #[test]
    fn test_0x24_bit_zero_page_not_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x24, 0xa1, 0x00]);
        cpu.reset();

        cpu.register_a = 0b0000_1111;
        cpu.memory[0xa1] = 0b1111_1111;

        cpu.run();
        assert!(!cpu.status.contains(Status::ZERO));
    }

    #[test]
    fn test_0x24_bit_zero_page_overflow_negative() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x24, 0xa1, 0x00]);
        cpu.reset();

        cpu.status.remove(Status::ZERO);

        cpu.register_a = 0b1100_0000;
        cpu.memory[0xa1] = 0b1100_0000;

        cpu.run();
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::OVERFLOW));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }
}