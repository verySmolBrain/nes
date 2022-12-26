#[cfg(test)]
mod test {
    use nes::cpu::CPU;

    #[test]
    fn test_a8_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa8, 0x00]);
        cpu.reset();

        cpu.register_a = 0x05;
        
        cpu.run();
        assert_eq!(cpu.register_y, 0x05)
    }
}