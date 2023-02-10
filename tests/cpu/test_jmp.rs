#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x4c_jmp_absolute() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x4c, 0x05, 0x00, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000; // addr: 0x8005 

        // addr: 0x8005 + 1 to reach break
        check(&mut cpu, expect![[r#"
            0000  4C 05 00  JMP $0005                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0005  00        BRK                             A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  9 CYC:3"#]])
    }

    #[test]
    fn test_0x6c_jmp_indirect_ff_bug() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x6c, 0xFF, 0x00, 0x00], 0x0000);

        // LSB: 0x80FF
        // MSB: 0x6c00
        bus.mem_write(0x00FF, 0x10);
        bus.mem_write(0x0000, 0x11);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000; // addr: 0x80FF

        /*
        An original 6502 has does not correctly fetch the target address if the 
        indirect vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF). 
        In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00.
        https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
        */

        // addr: 0x7677 + 1 to reach break
        check(&mut cpu, expect![[r#"
            0000  11 FF     ORA ($FF),Y = 1110 @ 1110 = 00  A:00 X:00 Y:00 P:24 SP:FD PPU:  0,  0 CYC:0
            0002  00        BRK                             A:00 X:00 Y:00 P:26 SP:FD PPU:  0, 15 CYC:5"#]])
    }
}