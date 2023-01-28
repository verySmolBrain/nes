#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::cpu::Status;
    use nes::bus::Bus;
    use nes::memory::Mem;
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

        cpu.register_a = 0b0000_1111;
        

        check(&mut cpu, expect![[r#"
            0000  24 A1     BIT $A1 = F0                    A:0F X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:0F X:00 Y:00 P:E6 SP:FD"#]])
    }

    #[test]
    fn test_0x24_bit_zero_page_not_zero() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x24, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0b1111_1111);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b0000_1111;

        check(&mut cpu, expect![[r#"
            0000  24 A1     BIT $A1 = FF                    A:0F X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:0F X:00 Y:00 P:E4 SP:FD"#]])
    }

    #[test]
    fn test_0x24_bit_zero_page_overflow_negative() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x24, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0b1100_0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.status.remove(Status::ZERO);
        cpu.register_a = 0b1100_0000;

        check(&mut cpu, expect![[r#"
            0000  24 A1     BIT $A1 = C0                    A:C0 X:00 Y:00 P:24 SP:FD
            0002  00        BRK                             A:C0 X:00 Y:00 P:E4 SP:FD"#]])
    }
}