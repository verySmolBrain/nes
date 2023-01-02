#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;
   
    #[test]
    fn test_0x29_and_immediate_base() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x29, 0b1111_0000, 0x00]);
        cpu.reset();

        cpu.register_a = 0b1000_1111;

        cpu.run();
        assert_eq!(0b1000_0000, cpu.register_a);
        assert_eq!(false, cpu.status.contains(Status::ZERO));
    }

    #[test]
    fn test_0x29_and_immediate_flags() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x29, 0b1111_0000, 0x00]);
        cpu.reset();

        cpu.register_a = 0b0000_1111;

        cpu.run();
        assert_eq!(0b0000_0000, cpu.register_a);
        assert_eq!(true, cpu.status.contains(Status::ZERO));
    }
}