#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use crate::helpers::check_zero_and_negative;

    #[test]
    fn test_aa_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xaa, 0x00]);
        cpu.reset();

        cpu.register_a = 0x05;
        
        cpu.run();
        assert_eq!(cpu.register_x, 0x05);
        check_zero_and_negative(cpu, 0x05);
    }
}

