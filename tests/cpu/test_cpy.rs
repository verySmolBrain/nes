#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;

    #[test]
    fn test_c0_immediate_a_greater() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xc0, 0x05, 0x00]);
        cpu.reset();

        cpu.register_y = 0x06; // cmp 0x06, 0x05
        
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_c0_immediate_a_less() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xc0, 0x06, 0x00]);
        cpu.reset();

        cpu.register_y = 0x05; // cmp 0x05, 0x06
        
        cpu.run();
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_c0_immediate_zero_zero() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xc0, 0x00, 0x00]);
        cpu.reset();

        cpu.register_y = 0x00; // cmp 0x00, 0x00
        
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
        assert!(cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_c0_immediate_test_big() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xc0, 0xff, 0x00]);
        cpu.reset();

        cpu.register_y = 0x01; // cmp 0xff, 0x01
    
        cpu.run();
        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));
    }
}