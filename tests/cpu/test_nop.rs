#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
   
    #[test]
    fn test_0xea_nop_nothin() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xea, 0x00]);
        cpu.reset();

        let initial_program_counter = cpu.program_counter;

        cpu.run();
        assert_eq!(cpu.program_counter, initial_program_counter + 2);
        // +1 for nop then +1 for next instruction
    }
}