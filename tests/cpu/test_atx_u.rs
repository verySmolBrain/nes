#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_ab_atx_u_immediate_set_transfer() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xab, 0b1000_1111, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0;
        cpu.register_a = 0b1111_1100;

        check(&mut cpu, expect![[r#"
            0000  AB 8F     ATX_U #$8F                      A:FC X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:8C X:8C Y:00 P:A4 SP:FD"#]])
    }
}