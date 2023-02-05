#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_9f_axa_u_absolute_y_write() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x9f, 0x10, 0x00, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.accumulator = 0b1111_1111;
        cpu.register_x = 0b0000_1111;

        check(&mut cpu, expect![[r#"
            0000  9F 10 00  AXA_U $0010,Y @ 0010 = 00       A:FF X:0F Y:00 P:24 SP:FD
            0003  00        BRK                             A:FF X:0F Y:00 P:24 SP:FD"#]]);

        let val = cpu.bus.mem_read(0x0010);
        let expected = expect!["00000111"];
        expected.assert_eq(format!("{:08b}", val).as_str());
    }
}