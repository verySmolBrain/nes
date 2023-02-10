#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_a7_lax_u_zero_page_load() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xa7, 0xa1, 0x00], 0x0000);
        bus.mem_write(0x00a1, 0b1);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;

        check(&mut cpu, expect![[r#"
            0000  A7 A1    *LAX $A1 = 01                    A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:01 X:01 Y:00 P:24 SP:FD PPU:  0,  9 CYC:3"#]]);
    }
}