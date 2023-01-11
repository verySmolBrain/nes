#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;
    use crate::helpers::check_zero_and_negative;

    #[test]
    fn test_69_adc_immediate_addition() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x69, 0x01, 0x00]);
        cpu.reset();

        cpu.register_a = 0;

        cpu.run();
        
        assert_eq!(cpu.register_a, 0x01);
        check_zero_and_negative(cpu, 0x01)
    }

    #[test]
    fn test_69_adc_immediate_big_addition() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x69, 120, 0x00]);
        cpu.reset();

        cpu.register_a = 120;

        cpu.run();
        
        assert_eq!(cpu.register_a, 240);
        assert!(cpu.status.contains(Status::OVERFLOW));
        check_zero_and_negative(cpu, 240);
    }

    #[test]
    fn test_69_adc_immediate_overflow() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x69, 80, 0x00]);
        cpu.reset();

        cpu.register_a = 80;

        cpu.run();
        
        assert_eq!(cpu.register_a, 160);
        assert!(cpu.status.contains(Status::OVERFLOW));
        check_zero_and_negative(cpu, 160);
    }

    #[test]
    fn test_69_adc_immediate_max_addition() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x69, 0xff, 0x00]);
        cpu.reset();

        cpu.register_a = 0xff;

        cpu.run();
        
        assert!(cpu.status.contains(Status::CARRY));
        assert_eq!(cpu.register_a, 0xfe);
        check_zero_and_negative(cpu, 0xfe)
    }
}