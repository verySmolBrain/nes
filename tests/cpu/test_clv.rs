#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;

    #[test]
    fn test_b8_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xb8, 0x00]);
        cpu.reset();

        cpu.status.insert(Status::OVERFLOW);
        assert!(cpu.status.contains(Status::OVERFLOW));
        
        cpu.run();
        assert!(!cpu.status.contains(Status::OVERFLOW));
    }
}