#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::cpu::Status;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x0a_asl_accumulator_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x0a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0b1000_0000;

        check(&mut cpu, expect![[r#"
            0000  0A        ASL A                           A:80 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0001  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_0x0a_asl_accumulator_no_set_from_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x0a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0b0000_0000;
        cpu.status.insert(Status::CARRY);
        
        check(&mut cpu, expect![[r#"
            0000  0A        ASL A                           A:00 X:00 Y:00 P:25 SP:FD PPU:  0,  0 CYC:0
            0001  00        BRK                             A:00 X:00 Y:00 P:26 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_0x0a_asl_accumulator_set_and_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x0a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0b1111_1000;
        cpu.status.remove(Status::CARRY);
        
        check(&mut cpu, expect![[r#"
            0000  0A        ASL A                           A:F8 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0001  00        BRK                             A:F0 X:00 Y:00 P:A5 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_0x06_asl_zero_page_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x06, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0b1000_0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        check(&mut cpu, expect![[r#"
            0000  06 A1     ASL $A1 = 80                    A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD PPU:  0, 15 CYC:5"#]])
    }
}