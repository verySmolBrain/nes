#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;

    #[test]
    fn test_c9_immediate_a_greater() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xc9, 0x05, 0x00]);
        cpu.reset();

        cpu.register_a = 0x06; // cmp 0x06, 0x05
        
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_c9_immediate_a_less() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xc9, 0x06, 0x00]);
        cpu.reset();

        cpu.register_a = 0x05; // cmp 0x05, 0x06
        
        cpu.run();
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_c9_immediate_zero_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xc9, 0x00, 0x00]);
        cpu.reset();

        cpu.register_a = 0x00; // cmp 0x00, 0x00
        
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_c9_immediate_test_big() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xc9, 0xff, 0x00]);
        cpu.reset();

        cpu.register_a = 0x01; // cmp 0xff, 0x01
    
        cpu.run();
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
    }
}