#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use nes::emulator::cpu::Status;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x6a_ror_accumulator_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x6a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b0000_0001;
        
        check(&mut cpu, expect![[r#"
            0000  6A        ROR A                           A:01 X:00 Y:00 P:24 SP:FD
            0001  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD"#]])
    }

    #[test]
    fn test_0x6a_ror_accumulator_set_from_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x6a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b0000_0000;
        cpu.status.insert(Status::CARRY);
        
        check(&mut cpu, expect![[r#"
            0000  6A        ROR A                           A:00 X:00 Y:00 P:25 SP:FD
            0001  00        BRK                             A:80 X:00 Y:00 P:A4 SP:FD"#]])
    }

    #[test]
    fn test_0x6a_ror_accumulator_set_and_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x6a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b1111_0001;
        cpu.status.insert(Status::CARRY);
        
        check(&mut cpu, expect![[r#"
            0000  6A        ROR A                           A:F1 X:00 Y:00 P:25 SP:FD
            0001  00        BRK                             A:F8 X:00 Y:00 P:A5 SP:FD"#]])
    }

    #[test]
    fn test_0x66_ror_zero_page_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x66, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0b0000_0001);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        
        check(&mut cpu, expect![[r#"
            0000  66 A1     ROR $A1 = 01                    A:00 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD"#]])
    }
}