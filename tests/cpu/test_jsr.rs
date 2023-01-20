#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;

    #[test]
    fn test_0x20_jsr_implied_push() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x20, 0x00]);
        cpu.reset();

        let old_pc = cpu.program_counter + 1;
        
        cpu.run();
        assert_eq!(old_pc + 2 - 1, cpu.stack_pop_u16());
    }
}