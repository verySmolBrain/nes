#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use nes::cpu::Status;
    use nes::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x0a_asl_accumulator_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x0a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b1000_0000;

        check(&mut cpu, expect![[""]])
    }

    #[test]
    fn test_0x0a_asl_accumulator_no_set_from_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x0a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b0000_0000;
        cpu.status.insert(Status::CARRY);
        
        check(&mut cpu, expect![[""]])
    }

    #[test]
    fn test_0x0a_asl_accumulator_set_and_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x0a, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_a = 0b1111_1000;
        cpu.status.remove(Status::CARRY);
        
        check(&mut cpu, expect![[""]])
    }

    #[test]
    fn test_0x06_asl_zero_page_carry() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x06, 0xa1, 0x00], 0x0000);
        bus.mem_write(0xa1, 0b1000_0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;

        check(&mut cpu, expect![[""]])
    }
}