#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use nes::memory::Mem;
    use crate::helpers::{ trace, TestRom };
    use expect_test::expect;

    #[test]
    fn test_format_trace() {
        let mut bus = Bus::new(TestRom::default_rom());
        bus.mem_write(0x00, 0xa2);
        bus.mem_write(0x01, 0x01);
        bus.mem_write(0x02, 0xca);
        bus.mem_write(0x03, 0x88);
        bus.mem_write(0x04, 0x00);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x00;
        cpu.register_a = 1;
        cpu.register_x = 2;
        cpu.register_y = 3;

        let mut res: Vec<String> = vec![];
        cpu.run_with_callback(|cpu| {
            res.push(trace(cpu));
        });

        let expected = expect![[r#"
            0000  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD
            0002  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD
            0003  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD
            0004  00        BRK                             A:01 X:00 Y:02 P:24 SP:FD"#]];
        expected.assert_eq(&res.join("\n"));
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

        let mut result: Vec<String> = vec![];
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu));
        });

        let expected = expect![[r#"
            0000  11 10     ORA ($10),Y = 0200 @ 0200 = AA  A:00 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:AA X:00 Y:00 P:A4 SP:FD"#]];
        expected.assert_eq(&result.join("\n"));
    }
}

/*
    Write tests for rom headers
    Write snapshot tests
 */