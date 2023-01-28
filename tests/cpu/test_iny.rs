#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_c8_immediate_zero() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc8, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_y = 0xff;
        
        check(&mut cpu, expect![[r#"
            0000  C8        INY                             A:00 X:00 Y:FF P:24 SP:FD
            0001  00        BRK                             A:00 X:00 Y:00 P:26 SP:FD"#]])
    }

    #[test]
    fn test_c8_immediate_one() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc8, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_y = 0;
        
        check(&mut cpu, expect![[r#"
            0000  C8        INY                             A:00 X:00 Y:00 P:24 SP:FD
            0001  00        BRK                             A:00 X:00 Y:01 P:24 SP:FD"#]])
    }
}