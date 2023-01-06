#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;

    #[test]
    fn test_0x0a_asl_accumulator_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x0a, 0x00]);
        cpu.reset();

        cpu.register_a = 0b1000_0000;
        assert!(!cpu.status.contains(Status::CARRY));
        
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
    }

    #[test]
    fn test_0x0a_asl_accumulator_no_set_from_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x0a, 0x00]);
        cpu.reset();

        cpu.register_a = 0b0000_0000;
        cpu.status.insert(Status::CARRY);
        
        cpu.run();
        assert_eq!(cpu.register_a, 0b0000_0000);
    }

    #[test]
    fn test_0x0a_asl_accumulator_set_and_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x0a, 0x00]);
        cpu.reset();

        cpu.register_a = 0b1111_1000;
        cpu.status.remove(Status::CARRY);
        
        cpu.run();
        assert_eq!(cpu.register_a, 0b1111_0000);
        assert!(cpu.status.contains(Status::CARRY));
    }

    #[test]
    fn test_0x06_asl_zero_page_carry() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x06, 0xa1, 0x00]);
        cpu.reset();

        cpu.memory[0xa1] = 0b1000_0000;
        assert!(!cpu.status.contains(Status::CARRY));
        
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
    }
}