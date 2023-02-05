#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_9b_xas_u_absolute_y_and() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x9b, 0x00, 0b0000_0011, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_x = 0b1111_1111;
        cpu.accumulator = 0b0000_1111;
        cpu.stack_pointer = 0b0000_0001;
        cpu.register_y = 0;
        
        check(&mut cpu, expect![[r#"
            0000  9B 00 03  XAS_U $0300,Y @ 0300 = 00       A:0F X:FF Y:00 P:24 SP:01
            0003  00        BRK                             A:0F X:FF Y:00 P:24 SP:0F"#]]);
        
        let expected = expect!["00000100"];
        expected.assert_eq(format!("{:08b}", cpu.mem_read(0b11_0000_0000)).as_str());
    }
}