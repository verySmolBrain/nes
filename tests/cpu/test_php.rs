#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;
   
    #[test]
    fn test_0x08_php_immediate() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x08, 0x00]);
        cpu.reset();

        cpu.status.insert(Status::ZERO);
        cpu.status.insert(Status::CARRY);
        cpu.status.insert(Status::NEGATIVE);

        cpu.run();
        let value = cpu.stack_pop_u8();
        let status = Status::from_bits_truncate(value);

        assert!(status.contains(Status::ZERO));
        assert!(status.contains(Status::CARRY));
        assert!(status.contains(Status::NEGATIVE));
        assert!(status.contains(Status::BREAKONE)); // Initial
        assert!(!status.contains(Status::DECIMAL));
        assert!(status.contains(Status::INTERDIS)); // Initial
        assert!(!status.contains(Status::OVERFLOW));
    }
}