#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x49_eor_immediate_base() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x49, 0b0000_0000, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0b1111_1111;

        check(&mut cpu, expect![[r#"
            0000  49 00     EOR #$00                        A:FF X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:FF X:00 Y:00 P:A4 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_0x49_eor_immediate_flags() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x49, 0b1111_0000, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0b1111_0000;

        check(&mut cpu, expect![[r#"
            0000  49 F0     EOR #$F0                        A:F0 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:00 X:00 Y:00 P:26 SP:FD PPU:  0,  6 CYC:2"#]])
    }
}