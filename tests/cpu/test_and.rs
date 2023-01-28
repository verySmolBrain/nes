#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x29_and_immediate_base() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x29, 0b1111_0000, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.register_a = 0b1000_1111;

        check(&mut cpu, expect![[r#"
            0000  29 F0     AND #$F0                        A:8F X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:80 X:00 Y:00 P:A4 SP:FD"#]])
    }

    #[test]
    fn test_0x29_and_immediate_flags() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x29, 0b1111_0000, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.register_a = 0b0000_1111;

        check(&mut cpu, expect![[r#"
            0000  29 F0     AND #$F0                        A:0F X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:00 X:00 Y:00 P:26 SP:FD"#]])
    }
}