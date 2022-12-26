#[cfg(test)]
mod test {
    use nes::cpu::CPU;

    #[test]
    fn test_ba_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xba, 0x00]);
        cpu.reset();

        cpu.stack_pointer = 0x05;
        
        cpu.run();
        assert_eq!(cpu.register_x, 0x05)
    }
}