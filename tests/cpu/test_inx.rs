#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use crate::helpers::check_zero_and_negative;

    #[test]
    fn test_e8_immediate_zero() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xe8, 0x00]);
        cpu.reset();

        cpu.register_x = 0xff;
        
        cpu.run();
        assert_eq!(cpu.register_x, 0x00);
        check_zero_and_negative(cpu, 0)
    }

    #[test]
    fn test_e8_immediate_one() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xe8, 0x00]);
        cpu.reset();

        cpu.register_x = 0;
        
        cpu.run();
        assert_eq!(cpu.register_x, 0x01);
        check_zero_and_negative(cpu, 1)
    }
}