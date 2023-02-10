#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Stack;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x68_pla_implied() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x68, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.stack_push_u8(0x05);
        
        check(&mut cpu, expect![[r#"
            0000  68        PLA                             A:00 X:00 Y:00 P:24 SP:FC PPU:  0,  0 CYC:0
            0001  00        BRK                             A:05 X:00 Y:00 P:24 SP:FD PPU:  0, 12 CYC:4"#]])
    }
}