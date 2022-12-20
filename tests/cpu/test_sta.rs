#[cfg(test)]
mod test {
    use nes::cpu::CPU;

    #[test]
    fn test_0x85_sta_zero_page() {
        let mut cpu = CPU::new();
        // Load accumulator into memory 0xa1
        cpu.load(vec![0x85, 0xa1, 0x00]);
        cpu.reset();
        cpu.register_a = 5;
        cpu.run();
        assert_eq!(cpu.mem_read(0xa1), 5);
    }
}