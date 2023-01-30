#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Stack;
    use crate::helpers::{ TestRom, load_into_memory };
    use expect_test::expect;

    #[test]
    fn test_stack_push_u8() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xa9, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        cpu.stack_push_u8(0x42);
        
        let expected = expect!["66"];
        expected.assert_eq(&cpu.stack_pop_u8().to_string());
    }

    #[test]
    fn test_stack_push_u16() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xa9, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        cpu.stack_push_u16(0x4042);

        let expected = expect!["16450"];
        expected.assert_eq(&cpu.stack_pop_u16().to_string());
    }
}