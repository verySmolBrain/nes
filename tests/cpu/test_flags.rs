#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

   #[test]
   fn test_0xa9_lda_negative() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xa9, 0xff, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
    
        check(&mut cpu, expect![[r#"
            0000  A9 FF     LDA #$FF                        A:00 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:FF X:00 Y:00 P:A4 SP:FD"#]])
    }
}