#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_cb_axs_u_immediate_write() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xcb, 0x5, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.accumulator = 0b1111_1111;
        cpu.register_x = 0b0000_1010;

        check(&mut cpu, expect![[r#"
            0000  CB 05     AXS_U #$05                      A:FF X:0A Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:FF X:05 Y:00 P:25 SP:FD PPU:  0,  6 CYC:2"#]]);
    }
}