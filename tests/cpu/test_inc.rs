#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use nes::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_e6_zero_page_zero() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xe6, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0xff);
        
        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        check(&mut cpu, expect![[""]])
    }

    #[test]
    fn test_e6_zero_page_one() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xe6, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0x00);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        check(&mut cpu, expect![[""]])
    }
}