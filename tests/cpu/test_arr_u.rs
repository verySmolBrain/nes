#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_6b_arr_u_immediate_bit6_bit7() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x6b, 0b0000_0110, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.register_a = 0b1111_1111;

        check(&mut cpu, expect![[r#"
            0000  6B 06     ARR_U #$06                      A:FF X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:03 X:00 Y:00 P:65 SP:FD"#]]);
    }

    #[test]
    fn test_6b_arr_u_immediate_bit6_not_bit7() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x6b, 0b0000_0100, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.register_a = 0b1111_1111;

        check(&mut cpu, expect![[r#"
            0000  6B 04     ARR_U #$04                      A:FF X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:02 X:00 Y:00 P:65 SP:FD"#]]);
    }

    #[test]
    fn test_6b_arr_u_immediate_not_bit6_bit7() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x6b, 0b0000_0010, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.register_a = 0b1111_1111;

        check(&mut cpu, expect![[r#"
            0000  6B 02     ARR_U #$02                      A:FF X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:01 X:00 Y:00 P:24 SP:FD"#]]);
    }

    #[test]
    fn test_6b_arr_u_immediate_not_bit6_not_bit7() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x6b, 0b0000_0000, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.register_a = 0b1111_1111;

        check(&mut cpu, expect![[r#"
            0000  6B 00     ARR_U #$00                      A:FF X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:00 X:00 Y:00 P:26 SP:FD"#]]);
    }
}