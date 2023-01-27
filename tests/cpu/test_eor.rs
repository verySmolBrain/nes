#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x49_eor_immediate_base() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x49, 0b0000_0000, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b1111_1111;

        check(&mut cpu, expect![[""]])
    }

    #[test]
    fn test_0x49_eor_immediate_flags() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x49, 0b1111_0000, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b1111_0000;

        check(&mut cpu, expect![[""]])
    }
}