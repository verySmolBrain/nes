#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_c7_dcp_u_zero_page_subtract() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc7, 0xa1, 0x00], 0x0000);
        bus.mem_write(0x00a1, 0x02);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;

        check(&mut cpu, expect![[r#"
            0000  C7 A1    *DCP $A1 = 02                    A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:00 X:00 Y:00 P:A4 SP:FD PPU:  0, 15 CYC:5"#]]);

        let val = cpu.bus.mem_read(0x00a1);
        let expected = expect![[r#"
            1
        "#]];
        expected.assert_debug_eq(&val);
    }
}