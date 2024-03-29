#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::cpu::Status;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x90_bcc_relative_no_branch() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x90, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.status.insert(Status::CARRY);
        assert!(cpu.status.contains(Status::CARRY));

        check(&mut cpu, expect![[r#"
            0000  90 05     BCC $07                         A:00 X:00 Y:00 P:25 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:00 X:00 Y:00 P:25 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_0x90_bcc_relative_branch() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x90, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.status.remove(Status::CARRY);
        assert!(!cpu.status.contains(Status::CARRY));

        check(&mut cpu, expect![[r#"
            0000  90 05     BCC $07                         A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0007  00        BRK                             A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  9 CYC:3"#]])
    }
}