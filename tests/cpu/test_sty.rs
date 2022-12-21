#[cfg(test)]
mod test {
    use nes::cpu::CPU;

    #[test]
    fn test_0x84_sty_zero_page() {
        let mut cpu = CPU::new();
        // Load register y into memory 0xa1
        cpu.load(vec![0x84, 0xa1, 0x00]);
        cpu.reset();
        cpu.register_y = 5;
        cpu.run();
        assert_eq!(cpu.mem_read(0xa1), 5);
    }
}