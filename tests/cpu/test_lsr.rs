#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use nes::cpu::Status;
    use nes::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x4a_lsr_accumulator_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x4a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b0000_0001;
        
        check(&mut cpu, expect![[r#"
            0000  4A        LSR A                           A:01 X:00 Y:00 P:24 SP:FD
            0001  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD"#]])
    }

    #[test]
    fn test_0x4a_lsr_accumulator_no_set_from_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x4a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b0000_0000;
        cpu.status.insert(Status::CARRY);
        
        check(&mut cpu, expect![[r#"
            0000  4A        LSR A                           A:00 X:00 Y:00 P:25 SP:FD
            0001  00        BRK                             A:00 X:00 Y:00 P:26 SP:FD"#]])
    }

    #[test]
    fn test_0x4a_lsr_accumulator_set_and_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x4a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b0001_1111;
        cpu.status.remove(Status::CARRY);
        
        check(&mut cpu, expect![[r#"
            0000  4A        LSR A                           A:1F X:00 Y:00 P:24 SP:FD
            0001  00        BRK                             A:0F X:00 Y:00 P:25 SP:FD"#]])
    }

    #[test]
    fn test_0x46_lsr_zero_page_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x46, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0b0000_0001);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        check(&mut cpu, expect![[r#"
            0000  46 A1     LSR $A1 = 01                    A:00 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD"#]])
    }
}