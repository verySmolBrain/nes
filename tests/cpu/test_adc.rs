#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_69_adc_immediate_addition() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x69, 0x01, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0;

        check(&mut cpu, expect![[r#"
            0000  69 01     ADC #$01                        A:00 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:01 X:00 Y:00 P:24 SP:FD"#]])
    }

    #[test]
    fn test_69_adc_immediate_big_addition() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x69, 120, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 120;

        check(&mut cpu, expect![[r#"
            0000  69 78     ADC #$78                        A:78 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:F0 X:00 Y:00 P:E4 SP:FD"#]])
    }

    #[test]
    fn test_69_adc_immediate_overflow() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x69, 80, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 80;
        
        check(&mut cpu, expect![[r#"
            0000  69 50     ADC #$50                        A:50 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:A0 X:00 Y:00 P:E4 SP:FD"#]])
    }

    #[test]
    fn test_69_adc_immediate_max_addition() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x69, 0xff, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0xff;

        check(&mut cpu, expect![[r#"
            0000  69 FF     ADC #$FF                        A:FF X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:FE X:00 Y:00 P:A5 SP:FD"#]])
    }
}