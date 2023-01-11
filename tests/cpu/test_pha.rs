#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
   
    #[test]
    fn test_0x48_pha_implied() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x48, 0x00]);
        cpu.reset();

        cpu.register_a = 0x05;

        cpu.run();
        assert_eq!(0x05, cpu.stack_pop_u8());
    }
}