#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;

    #[test]
    fn test_18_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x18, 0x00]);
        cpu.reset();

        cpu.status.insert(Status::CARRY);
        assert!(cpu.status.contains(Status::CARRY));
        
        cpu.run();
        assert!(!cpu.status.contains(Status::CARRY));
    }
}