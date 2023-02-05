#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_1a_nop_nothin() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x1a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        // +1 for nop then +1 for next instruction
        check(&mut cpu, expect![[r#"
            0000  1A       *NOP                             A:00 X:00 Y:00 P:24 SP:FD
            0001  00        BRK                             A:00 X:00 Y:00 P:24 SP:FD"#]])
    }
}