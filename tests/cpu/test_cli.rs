#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;

    #[test]
    fn test_58_none() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x58, 0x00]);
        cpu.reset();

        cpu.status.insert(Status::INTERDIS);
        assert!(cpu.status.contains(Status::INTERDIS));
        
        cpu.run();
        assert!(!cpu.status.contains(Status::INTERDIS));
    }
}