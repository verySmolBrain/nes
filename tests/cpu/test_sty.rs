#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::memory::Mem;

    #[test]
    fn test_0x84_sty_zero_page() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        // Load register y into memory 0xa1
        cpu.load(vec![0x84, 0xa1, 0x00]);
        cpu.reset();
        cpu.register_y = 5;
        cpu.run();
        assert_eq!(cpu.mem_read(0xa1), 5);
    }
}