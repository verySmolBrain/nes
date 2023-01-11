#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;

    #[test]
    fn test_0xf8_none_set() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xf8, 0x00]);
        cpu.reset();
        assert!(!cpu.status.contains(Status::DECIMAL));
        cpu.run();
        assert!(cpu.status.contains(Status::DECIMAL));
    }
}