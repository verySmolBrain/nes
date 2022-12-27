#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;

    #[test]
    fn test_d8_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xd8, 0x00]);
        cpu.reset();

        cpu.status.insert(Status::DECIMAL);
        assert!(cpu.status.contains(Status::DECIMAL));
        
        cpu.run();
        assert!(!cpu.status.contains(Status::DECIMAL));
    }
}