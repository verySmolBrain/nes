#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_c9_immediate_a_greater() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc9, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0x06; // cmp 0x06, 0x05
        
        check(&mut cpu, expect![[""]])
    }

    #[test]
    fn test_c9_immediate_a_less() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc9, 0x06, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0x05; // cmp 0x05, 0x06
        
        check(&mut cpu, expect![[""]])
    }

    #[test]
    fn test_c9_immediate_zero_zero() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc9, 0x00, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0x00; // cmp 0x00, 0x00
        
        check(&mut cpu, expect![[""]])
    }

    #[test]
    fn test_c9_immediate_test_big() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xc9, 0xff, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0x01; // cmp 0xff, 0x01
    
        check(&mut cpu, expect![[""]])
    }
}