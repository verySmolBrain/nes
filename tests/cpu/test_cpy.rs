#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_c0_immediate_a_greater() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc0, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_y = 0x06; // cmp 0x06, 0x05
        
        check(&mut cpu, expect![[r#"
            0000  C0 05     CPY #$05                        A:00 X:00 Y:06 P:24 SP:FD
            0002  00        BRK                             A:00 X:00 Y:06 P:25 SP:FD"#]])
    }

    #[test]
    fn test_c0_immediate_a_less() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc0, 0x06, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_y = 0x05; // cmp 0x05, 0x06
        
        check(&mut cpu, expect![[r#"
            0000  C0 06     CPY #$06                        A:00 X:00 Y:05 P:24 SP:FD
            0002  00        BRK                             A:00 X:00 Y:05 P:A4 SP:FD"#]])
    }

    #[test]
    fn test_c0_immediate_zero_zero() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc0, 0x00, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_y = 0x00; // cmp 0x00, 0x00
        
        check(&mut cpu, expect![[r#"
            0000  C0 00     CPY #$00                        A:00 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD"#]])
    }

    #[test]
    fn test_c0_immediate_test_big() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc0, 0xff, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_y = 0x01; // cmp 0xff, 0x01
    
        check(&mut cpu, expect![[r#"
            0000  C0 FF     CPY #$FF                        A:00 X:00 Y:01 P:24 SP:FD
            0002  00        BRK                             A:00 X:00 Y:01 P:24 SP:FD"#]])
    }
}