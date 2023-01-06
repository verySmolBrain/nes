#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;
   
    #[test]
    fn test_0x70_bvs_relative_no_branch() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x70, 0x05, 0x00]);
        cpu.reset();

        cpu.status.remove(Status::OVERFLOW);
        assert!(!cpu.status.contains(Status::OVERFLOW));

        cpu.run();
        assert_eq!(0x8003, cpu.program_counter);
    }

    #[test]
    fn test_0x70_bvs_relative_branch() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x70, 0x05, 0x00]);
        cpu.reset();

        cpu.status.insert(Status::OVERFLOW);
        assert!(cpu.status.contains(Status::OVERFLOW));

        cpu.run(); // 0x8001 + 0x05 (Relative) + 0x1 (Skip Label) + 0x1 (Next instruction)
        assert_eq!(0x8008, cpu.program_counter);
    }
}