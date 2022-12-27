#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;

    #[test]
    fn test_58_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x58, 0x00]);
        cpu.reset();

        cpu.status.insert(Status::INTERDIS);
        assert!(cpu.status.contains(Status::INTERDIS));
        
        cpu.run();
        assert!(!cpu.status.contains(Status::INTERDIS));
    }
}