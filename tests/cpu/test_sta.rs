#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x85_sta_zero_page() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x85, 0xa1, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 5;
        
        check(&mut cpu, expect![[r#"
            0000  85 A1     STA $A1 = 00                    A:05 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:05 X:00 Y:00 P:24 SP:FD"#]])
    }
}