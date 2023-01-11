#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;
    use nes::cpu::ROM_START;
   
    #[test]
    fn test_0xf0_beq_relative_no_branch() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xf0, 0x05, 0x00]);
        cpu.reset();

        assert!(!cpu.status.contains(Status::ZERO));

        cpu.run();
        assert_eq!((ROM_START as u16) + 3, cpu.program_counter);
    }

    #[test]
    fn test_0xf0_beq_relative_branch() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xf0, 0x05, 0x00]);
        cpu.reset();

        cpu.status.insert(Status::ZERO);
        assert!(cpu.status.contains(Status::ZERO));

        cpu.run(); // 0x8001 + 0x05 (Relative) + 0x1 (Skip Label) + 0x1 (Next instruction)
        assert_eq!((ROM_START as u16) + 8, cpu.program_counter);
    }
}