#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;

    #[test]
    fn test_0x4a_lsr_accumulator_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x4a, 0x00]);
        cpu.reset();

        cpu.register_a = 0b0000_0001;
        assert!(!cpu.status.contains(Status::CARRY));
        
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
    }

    #[test]
    fn test_0x4a_lsr_accumulator_no_set_from_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x4a, 0x00]);
        cpu.reset();

        cpu.register_a = 0b0000_0000;
        cpu.status.insert(Status::CARRY);
        
        cpu.run();
        assert_eq!(cpu.register_a, 0b0000_0000);
        assert!(cpu.status.contains(Status::ZERO))
    }

    #[test]
    fn test_0x4a_lsr_accumulator_set_and_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x4a, 0x00]);
        cpu.reset();

        cpu.register_a = 0b0001_1111;
        cpu.status.remove(Status::CARRY);
        
        cpu.run();
        assert_eq!(cpu.register_a, 0b0000_1111);
        assert!(cpu.status.contains(Status::CARRY));
    }

    #[test]
    fn test_0x46_lsr_zero_page_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x46, 0xa1, 0x00]);
        cpu.reset();

        cpu.memory[0xa1] = 0b0000_0001;
        assert!(!cpu.status.contains(Status::CARRY));
        
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
    }
}