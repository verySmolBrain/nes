#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_c9_immediate_a_greater() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc9, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0x06; // cmp 0x06, 0x05
        
        check(&mut cpu, expect![[r#"
            0000  C9 05     CMP #$05                        A:06 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:06 X:00 Y:00 P:25 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_c9_immediate_a_less() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc9, 0x06, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0x05; // cmp 0x05, 0x06
        
        check(&mut cpu, expect![[r#"
            0000  C9 06     CMP #$06                        A:05 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:05 X:00 Y:00 P:A4 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_c9_immediate_zero_zero() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc9, 0x00, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0x00; // cmp 0x00, 0x00
        
        check(&mut cpu, expect![[r#"
            0000  C9 00     CMP #$00                        A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_c9_immediate_test_big() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc9, 0xff, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0x01; // cmp 0xff, 0x01
    
        check(&mut cpu, expect![[r#"
            0000  C9 FF     CMP #$FF                        A:01 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:01 X:00 Y:00 P:24 SP:FD PPU:  0,  6 CYC:2"#]])
    }
}