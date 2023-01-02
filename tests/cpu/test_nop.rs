#[cfg(test)]
mod test {
    use nes::cpu::CPU;
   
    #[test]
    fn test_0xea_nop_nothin() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xea, 0x00]);
        cpu.reset();

        let initial_program_counter = cpu.program_counter;

        cpu.run();
        assert_eq!(cpu.program_counter, initial_program_counter + 2);
        // +1 for nop then +1 for next instruction
    }
}