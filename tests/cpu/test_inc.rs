#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_e6_zero_page_zero() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xe6, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0xff);
        
        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        check(&mut cpu, expect![[r#"
            0000  E6 A1     INC $A1 = FF                    A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:00 X:00 Y:00 P:26 SP:FD PPU:  0, 15 CYC:5"#]])
    }

    #[test]
    fn test_e6_zero_page_one() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xe6, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0x00);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        check(&mut cpu, expect![[r#"
            0000  E6 A1     INC $A1 = 00                    A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 15 CYC:5"#]])
    }
}