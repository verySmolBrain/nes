#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use crate::helpers::check_zero_and_negative;
    // SUS
    #[test]
    fn test_e9_sbc_immediate_subtract() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x00, 0x00]);
        cpu.reset();

        cpu.register_a = 1;

        cpu.run();
        
        assert_eq!(cpu.register_a, 0x00);
        check_zero_and_negative(cpu, 0x00)
    }

    #[test]
    fn test_e9_sbc_immediate_subtract_another() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x06, 0x00]);
        cpu.reset();

        cpu.register_a = 0x09;

        cpu.run();
        
        assert_eq!(cpu.register_a, 0x02);
        check_zero_and_negative(cpu, 0x02)
    }

    #[test]
    fn test_e9_sbc_immediate_subtract_bigger() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 112, 0x00]);
        cpu.reset();

        cpu.register_a = 80;

        cpu.run();
        
        assert_eq!(cpu.register_a, 0b1101_1111);
        check_zero_and_negative(cpu, 0b1101_1111);
    }

    #[test]
    fn test_e9_sbc_immediate_subtract_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 80, 0x00]);
        cpu.reset();

        cpu.register_a = 80;

        cpu.run();
        
        assert_eq!(cpu.register_a, 0xff);
        check_zero_and_negative(cpu, 0xff);
    }

    #[test]
    fn test_e9_sbc_immediate_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe9, 0x00, 0x00]);
        cpu.reset();

        cpu.register_a = 0xff;

        cpu.run();
        
        assert_eq!(cpu.register_a, 0xfe);
        check_zero_and_negative(cpu, 0xfe)
    }
}