#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_aa_tax_none_move() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xaa, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0x05;
        
        check(&mut cpu, expect![[r#"
            0000  AA        TAX                             A:05 X:00 Y:00 P:24 SP:FD
            0001  00        BRK                             A:05 X:05 Y:00 P:24 SP:FD"#]])
    }
}

