#[cfg(test)]
mod test {
    use nes::cpu::CPU;
   
    #[test]
    fn test_0x48_pha_implied() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x48, 0x00]);
        cpu.reset();

        cpu.register_a = 0x05;

        cpu.run();
        assert_eq!(0x05, cpu.stack_pop_u8());
    }
}