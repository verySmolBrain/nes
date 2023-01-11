#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;
   
    #[test]
    fn test_0x09_ora_immediate_base() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x09, 0b1111_0000, 0x00]);
        cpu.reset();

        cpu.register_a = 0b0000_1111;

        cpu.run();
        assert_eq!(0b1111_1111, cpu.register_a);
        assert_eq!(false, cpu.status.contains(Status::ZERO));
    }

    #[test]
    fn test_0x09_ora_immediate_flags() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x09, 0b0000_0000, 0x00]);
        cpu.reset();

        cpu.register_a = 0b0000_0000;

        cpu.run();
        assert_eq!(0b0000_0000, cpu.register_a);
        assert_eq!(true, cpu.status.contains(Status::ZERO));
    }
}