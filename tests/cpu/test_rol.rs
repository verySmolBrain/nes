#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use nes::emulator::cpu::Status;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x2a_rol_accumulator_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x2a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0b1000_0000;
        
        check(&mut cpu, expect![[r#"
            0000  2A        ROL A                           A:80 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0001  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_0x2a_rol_accumulator_set_from_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x2a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0b0000_0000;
        cpu.status.insert(Status::CARRY);
        
        check(&mut cpu, expect![[r#"
            0000  2A        ROL A                           A:00 X:00 Y:00 P:25 SP:FD PPU:  0,  0 CYC:0
            0001  00        BRK                             A:01 X:00 Y:00 P:24 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_0x2a_rol_accumulator_set_and_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x2a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0b1000_1111;
        cpu.status.insert(Status::CARRY);
        
        check(&mut cpu, expect![[r#"
            0000  2A        ROL A                           A:8F X:00 Y:00 P:25 SP:FD PPU:  0,  0 CYC:0
            0001  00        BRK                             A:1F X:00 Y:00 P:25 SP:FD PPU:  0,  6 CYC:2"#]])
    }

    #[test]
    fn test_0x26_rol_zero_page_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x26, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0b1000_0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        
        check(&mut cpu, expect![[r#"
            0000  26 A1     ROL $A1 = 80                    A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:00 X:00 Y:00 P:27 SP:FD PPU:  0, 15 CYC:5"#]])
    }
}