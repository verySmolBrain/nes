#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::cpu::Status;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x24_bit_zero_page_zero() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x24, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0b1111_0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.status.remove(Status::ZERO);

        cpu.accumulator = 0b0000_1111;
        

        check(&mut cpu, expect![[r#"
            0000  24 A1     BIT $A1 = F0                    A:0F X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:0F X:00 Y:00 P:E6 SP:FD PPU:  0,  9 CYC:3"#]])
    }

    #[test]
    fn test_0x24_bit_zero_page_not_zero() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x24, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0b1111_1111);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.accumulator = 0b0000_1111;

        check(&mut cpu, expect![[r#"
            0000  24 A1     BIT $A1 = FF                    A:0F X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:0F X:00 Y:00 P:E4 SP:FD PPU:  0,  9 CYC:3"#]])
    }

    #[test]
    fn test_0x24_bit_zero_page_overflow_negative() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x24, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0b1100_0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.status.remove(Status::ZERO);
        cpu.accumulator = 0b1100_0000;

        check(&mut cpu, expect![[r#"
            0000  24 A1     BIT $A1 = C0                    A:C0 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:C0 X:00 Y:00 P:E4 SP:FD PPU:  0,  9 CYC:3"#]])
    }
}