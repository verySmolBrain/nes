#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
    // SUS
    #[test]
    fn test_e9_sbc_immediate_subtract() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xe9, 0x00, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 1;

        check(&mut cpu, expect![[r#"
            0000  E9 00     SBC #$00                        A:01 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD"#]])
    }

    #[test]
    fn test_e9_sbc_immediate_subtract_another() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xe9, 0x06, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0x09;

        check(&mut cpu, expect![[r#"
            0000  E9 06     SBC #$06                        A:09 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:02 X:00 Y:00 P:25 SP:FD"#]])
    }

    #[test]
    fn test_e9_sbc_immediate_subtract_bigger() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xe9, 112, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 80;

        check(&mut cpu, expect![[r#"
            0000  E9 70     SBC #$70                        A:50 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:DF X:00 Y:00 P:A4 SP:FD"#]])
    }

    #[test]
    fn test_e9_sbc_immediate_subtract_zero() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xe9, 80, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 80;

        check(&mut cpu, expect![[r#"
            0000  E9 50     SBC #$50                        A:50 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:FF X:00 Y:00 P:A4 SP:FD"#]])
    }

    #[test]
    fn test_e9_sbc_immediate_overflow() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xe9, 0x00, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0xff;

        check(&mut cpu, expect![[r#"
            0000  E9 00     SBC #$00                        A:FF X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:FE X:00 Y:00 P:A5 SP:FD"#]])
    }
}