#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Stack;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x20_jsr_implied_push() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x20, 0x00], 0x0005);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0005;

        check(&mut cpu, expect![[r#"
            0005  20 00 00  JSR $0000                       A:00 X:00 Y:00 P:24 SP:FD
            0000  00        BRK                             A:00 X:00 Y:00 P:24 SP:FB"#]]);
        
        let expected = expect!["7"];
        expected.assert_eq(&cpu.stack_pop_u16().to_string())
    }
}