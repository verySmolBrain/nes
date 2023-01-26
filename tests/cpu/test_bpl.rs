#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::cpu::Status;
    use nes::bus::Bus;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x10_bpl_relative_no_branch() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x10, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.status.insert(Status::NEGATIVE);
        assert!(cpu.status.contains(Status::NEGATIVE));

        check(&mut cpu, expect![[""]])
    }

    #[test]
    fn test_0x10_bpl_relative_branch() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x10, 0x05, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.status.remove(Status::NEGATIVE);
        assert!(!cpu.status.contains(Status::NEGATIVE));

        // 0x8001 + 0x05 (Relative) + 0x1 (Skip Label) + 0x1 (Next instruction)
        check(&mut cpu, expect![[""]])
    }
}