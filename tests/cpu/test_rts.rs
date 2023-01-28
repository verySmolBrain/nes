#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x60_rts_implied_pop() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x60, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        let addr = 0x4042_u16.wrapping_sub(1);
        cpu.stack_push_u16(addr);
        
        // 0x4042 + 1 for next
        check(&mut cpu, expect![[""]])
    }
}