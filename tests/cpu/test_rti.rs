#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;

    #[test]
    fn test_0x40_rti_implied_pop() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x40, 0x00]);
        cpu.reset();

        let addr = 0x4042_u16;
        cpu.stack_push_u16(addr);

        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        let new_status = Status::CARRY | Status::ZERO | Status::NEGATIVE;
        cpu.stack_push_u8(new_status.bits());
        
        cpu.run();
        
        assert_eq!(0x4042 + 1, cpu.program_counter);
        assert!(cpu.status.contains(Status::CARRY));
        assert!(cpu.status.contains(Status::ZERO));
        assert!(cpu.status.contains(Status::NEGATIVE));
    }
}