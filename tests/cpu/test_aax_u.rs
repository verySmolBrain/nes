#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_87_aax_u_absolute_and() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x87, 0xa1, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.accumulator = 0b0001_1111;
        cpu.register_x = 0b1111_1000;

        check(&mut cpu, expect![[r#"
            0000  87 A1     AAX_U $A1 = 00                  A:1F X:F8 Y:00 P:24 SP:FD
            0002  00        BRK                             A:1F X:F8 Y:00 P:24 SP:FD"#]]);

        let val = cpu.mem_read(0x00a1);
        let expected = expect!["00011000"];
        expected.assert_eq(&format!("{:08b}", val).as_str());
    }
}