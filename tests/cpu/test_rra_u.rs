#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_67_rra_zero_page_rotate_add() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x67, 0xa1, 0x00], 0x0000);
        bus.mem_write(0x00a1, 0b0000_0011);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        check(&mut cpu, expect![[r#"
            0000  67 A1    *RRA $A1 = 03                    A:00 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:02 X:00 Y:00 P:24 SP:FD"#]])
    }
}