#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use crate::helpers::check_zero_and_negative;

    #[test]
    fn test_ca_none_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xca, 0x00]);
        cpu.reset();

        cpu.register_x = 1;

        cpu.run();
        
        assert_eq!(cpu.register_x, 0x00);
        check_zero_and_negative(cpu, 0)
    }
}