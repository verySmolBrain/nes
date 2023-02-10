#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_format_trace() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xa2, 0x01, 0xca, 0x88, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x00;
        cpu.accumulator = 1;
        cpu.register_x = 2;
        cpu.register_y = 3;

        check(&mut cpu, expect![[r#"
            0000  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD PPU:  0,  6 CYC:2
            0003  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD PPU:  0, 12 CYC:4
            0004  00        BRK                             A:01 X:00 Y:02 P:24 SP:FD PPU:  0, 18 CYC:6"#]])
    }

    #[test]
    fn test_format_trace_memory_access() {
        let mut bus = Bus::new(TestRom::default_rom());
        bus.mem_write(0x00, 0x11);
        bus.mem_write(0x01, 0x10);
        bus.mem_write(0x10, 0x00);
        bus.mem_write(0x11, 0x02);
        bus.mem_write(0x0200, 0xAA);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x00;
        cpu.register_y = 0;

        check(&mut cpu, expect![[r#"
            0000  11 10     ORA ($10),Y = 0200 @ 0200 = AA  A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:AA X:00 Y:00 P:A4 SP:FD PPU:  0, 15 CYC:5"#]])
    }
}