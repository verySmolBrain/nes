#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use nes::cpu::Status;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x40_rti_implied_pop() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x40, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        let addr = 0x4042_u16;
        cpu.stack_push_u16(addr);

        assert!(!cpu.status.contains(Status::CARRY));
        assert!(!cpu.status.contains(Status::ZERO));
        assert!(!cpu.status.contains(Status::NEGATIVE));

        let new_status = Status::CARRY | Status::ZERO | Status::NEGATIVE;
        cpu.stack_push_u8(new_status.bits());
        
        check(&mut cpu, expect![[r#"
            0000  40        RTI                             A:00 X:00 Y:00 P:24 SP:FA
            4042  00        BRK                             A:00 X:00 Y:00 P:A3 SP:FD"#]])
    }
}