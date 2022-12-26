#[cfg(test)]
mod test {
    use nes::cpu::CPU;

    #[test]
    fn test_9a_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x9a, 0x00]);
        cpu.reset();

        cpu.register_x = 0x05;
        
        cpu.run();
        assert_eq!(cpu.stack_pointer, 0x05)
    }
}