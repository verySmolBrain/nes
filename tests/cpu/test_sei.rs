#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::cpu::Status;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x78_none_set() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x78, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;


        assert!(cpu.status.contains(Status::INTERDIS)); // INTERDIS is initial true
        cpu.status.remove(Status::INTERDIS);
        assert!(!cpu.status.contains(Status::INTERDIS));
        
        check(&mut cpu, expect![[r#"
            0000  78        SEI                             A:00 X:00 Y:00 P:20 SP:FD
            0001  00        BRK                             A:00 X:00 Y:00 P:24 SP:FD"#]])
    }
}