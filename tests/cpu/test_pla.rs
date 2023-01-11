#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
   
    #[test]
    fn test_0x68_pla_implied() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x68, 0x00]);
        cpu.reset();

        cpu.stack_push_u8(0x05);
        assert_eq!(cpu.register_a, 0);

        cpu.run();
        assert_eq!(cpu.register_a, 0x05);
    }
}