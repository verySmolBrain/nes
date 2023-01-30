#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x86_stx_zero_page() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x86, 0xa1, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_x = 5;
        
        check(&mut cpu, expect![[r#"
            0000  86 A1     STX $A1 = 00                    A:00 X:05 Y:00 P:24 SP:FD
            0002  00        BRK                             A:00 X:05 Y:00 P:24 SP:FD"#]])
    }
}