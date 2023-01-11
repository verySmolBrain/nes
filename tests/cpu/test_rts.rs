#[cfg(test)]
mod test {
    use nes::bus::Bus;
    use nes::cpu::CPU;

    #[test]
    fn test_0x60_rts_implied_pop() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x60, 0x00]);
        cpu.reset();

        let addr = 0x4042_u16.wrapping_sub(1);
        cpu.stack_push_u16(addr);
        
        cpu.run(); // + 1 for next
        assert_eq!(0x4042 + 1, cpu.program_counter);
    }
}