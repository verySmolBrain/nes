#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use nes::cpu::Status;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x28_php_immediate() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x28, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        let mut status = Status::empty();
        status.insert(Status::ZERO);
        status.insert(Status::CARRY);
        status.insert(Status::NEGATIVE);
        status.insert(Status::BREAKONE);
        status.insert(Status::INTERDIS);

        cpu.stack_push_u8(status.bits());

        check(&mut cpu, expect![[r#"
            0000  28        PLP                             A:00 X:00 Y:00 P:24 SP:FC
            0001  00        BRK                             A:00 X:00 Y:00 P:A7 SP:FD"#]])
    }
}