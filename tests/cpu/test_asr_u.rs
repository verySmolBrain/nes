#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_4b_asr_u_immediate_rotate() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x4b, 0b0000_0010, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.accumulator = 0b1111_1111;

        check(&mut cpu, expect![[r#"
            0000  4B 02     ASR_U #$02                      A:FF X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:01 X:00 Y:00 P:24 SP:FD PPU:  0,  6 CYC:2"#]]);
    }

    #[test]
    fn test_4b_asr_u_immediate_rotate_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x4b, 0b0000_0001, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.accumulator = 0b1111_1111;

        check(&mut cpu, expect![[r#"
            0000  4B 01     ASR_U #$01                      A:FF X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD PPU:  0,  6 CYC:2"#]]);
    }
}