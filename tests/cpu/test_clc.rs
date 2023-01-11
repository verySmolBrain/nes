#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;

    #[test]
    fn test_18_none() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x18, 0x00]);
        cpu.reset();

        cpu.status.insert(Status::CARRY);
        assert!(cpu.status.contains(Status::CARRY));
        
        cpu.run();
        assert!(!cpu.status.contains(Status::CARRY));
    }
}