#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_e7_isc_u_zero_page_inc_1() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xe7, 0xa1, 0x00], 0x0000);
        bus.mem_write(0x00a1, 0x01);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.accumulator = 5;

        check(&mut cpu, expect![[r#"
            0000  E7 A1    *ISB $A1 = 01                    A:05 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:02 X:00 Y:00 P:25 SP:FD"#]]);
    }
}