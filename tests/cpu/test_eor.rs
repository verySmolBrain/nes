#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;
   
    #[test]
    fn test_0x49_eor_immediate_base() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x49, 0b0000_0000, 0x00]);
        cpu.reset();

        cpu.register_a = 0b1111_1111;

        cpu.run();
        assert_eq!(0b1111_1111, cpu.register_a);
        assert_eq!(false, cpu.status.contains(Status::ZERO));
    }

    #[test]
    fn test_0x49_eor_immediate_flags() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x49, 0b1111_0000, 0x00]);
        cpu.reset();

        cpu.register_a = 0b1111_0000;

        cpu.run();
        assert_eq!(0b0000_0000, cpu.register_a);
        assert_eq!(true, cpu.status.contains(Status::ZERO));
    }
}