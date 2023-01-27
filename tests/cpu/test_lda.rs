#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xa9, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        check(&mut cpu, expect![[""]])
    }
   
    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0xa9, 0x00, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        check(&mut cpu, expect![[""]])
    }
}