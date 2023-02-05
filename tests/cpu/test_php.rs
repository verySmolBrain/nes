#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::cpu::Status;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x08_php_immediate() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x08, 0x28, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.status.insert(Status::ZERO);
        cpu.status.insert(Status::CARRY);
        cpu.status.insert(Status::NEGATIVE);

        check(&mut cpu, expect![[r#"
            0000  08        PHP                             A:00 X:00 Y:00 P:A7 SP:FD
            0001  28        PLP                             A:00 X:00 Y:00 P:A7 SP:FC
            0002  00        BRK                             A:00 X:00 Y:00 P:A7 SP:FD"#]])
    }
}