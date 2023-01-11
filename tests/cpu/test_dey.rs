#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use crate::helpers::check_zero_and_negative;

    #[test]
    fn test_88_none_zero() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x88, 0x00]);
        cpu.reset();

        cpu.register_y = 1;

        cpu.run();
        
        assert_eq!(cpu.register_y, 0x00);
        check_zero_and_negative(cpu, 0)
    }
}