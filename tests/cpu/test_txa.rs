#[cfg(test)]
mod test {
    use nes::cpu::CPU;

    #[test]
    fn test_8a_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x8a, 0x00]);
        cpu.reset();

        cpu.register_x = 0x05;
        
        cpu.run();
        assert_eq!(cpu.register_a, 0x05)
    }
}