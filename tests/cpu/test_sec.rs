#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;

    #[test]
    fn test_0x38_none_set() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x38, 0x00]);
        cpu.reset();
        assert!(!cpu.status.contains(Status::CARRY));
        cpu.run();
        assert!(cpu.status.contains(Status::CARRY));
    }
}