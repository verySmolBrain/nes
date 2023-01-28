#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::cpu::Status;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0xb0_bcs_relative_no_branch() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xb0, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        assert!(!cpu.status.contains(Status::CARRY));

        check(&mut cpu, expect![[r#"
            0000  B0 05     BCS $07                         A:00 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:00 X:00 Y:00 P:24 SP:FD"#]])
    }

    #[test]
    fn test_0xb0_bcs_relative_branch() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xb0, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.status.insert(Status::CARRY);
        assert!(cpu.status.contains(Status::CARRY));

        // 0x8001 + 0x05 (Relative) + 0x1 (Skip Label) + 0x1 (Next instruction)
        check(&mut cpu, expect![[r#"
            0000  B0 05     BCS $07                         A:00 X:00 Y:00 P:25 SP:FD
            0007  00        BRK                             A:00 X:00 Y:00 P:25 SP:FD"#]])
    }
}