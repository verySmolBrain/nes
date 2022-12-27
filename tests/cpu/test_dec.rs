#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use crate::helpers::check_zero_and_negative;

    #[test]
    fn test_c6_zero_page_zero() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xc6, 0xa1, 0x00]);
        cpu.reset();

        cpu.mem_write(0xa1, 0x01);

        cpu.run();
        
        assert_eq!(cpu.mem_read(0xa1), 0x00);
        check_zero_and_negative(cpu, 0)
    }
}