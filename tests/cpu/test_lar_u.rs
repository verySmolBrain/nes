#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_bb_lar_u_zero_page_load() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xbb, 0xa1, 0x00], 0x0000);
        bus.mem_write(0x00a1, 0b1111_1000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.stack_pointer = 0b0001_1111;

        check(&mut cpu, expect![[r#"
            0000  BB A1 00  LAR_U $00A1,Y @ 00A1 = F8       A:00 X:00 Y:00 P:24 SP:1F
            0003  00        BRK                             A:18 X:18 Y:00 P:24 SP:18"#]]);
    }
}