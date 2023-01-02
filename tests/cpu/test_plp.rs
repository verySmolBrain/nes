#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;
   
    #[test]
    fn test_0x28_php_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x28, 0x00]);
        cpu.reset();

        let mut status = Status::empty();
        status.insert(Status::ZERO);
        status.insert(Status::CARRY);
        status.insert(Status::NEGATIVE);
        status.insert(Status::BREAKONE);
        status.insert(Status::INTERDIS);

        cpu.stack_push_u8(status.bits());

        cpu.run();

        assert!(status.contains(Status::ZERO));
        assert!(status.contains(Status::CARRY));
        assert!(status.contains(Status::NEGATIVE));
        assert!(status.contains(Status::BREAKONE)); // Initial
        assert!(!status.contains(Status::DECIMAL));
        assert!(status.contains(Status::INTERDIS)); // Initial
        assert!(!status.contains(Status::OVERFLOW));
    }
}