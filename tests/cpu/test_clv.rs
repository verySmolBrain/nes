#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use nes::cpu::Status;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_b8_none() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xb8, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.status.insert(Status::OVERFLOW);
        assert!(cpu.status.contains(Status::OVERFLOW));
        
        check(&mut cpu, expect![[r#"
            0000  B8        CLV                             A:00 X:00 Y:00 P:64 SP:FD
            0001  00        BRK                             A:00 X:00 Y:00 P:24 SP:FD"#]])
    }
}