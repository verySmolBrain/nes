#[cfg(test)]
mod test {
    use nes::cpu::Cpu;
    use nes::bus::Bus;
    use nes::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;
   
    #[test]
    fn test_0x4c_jmp_absolute() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x4c, 0x05, 0x00, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000; // addr: 0x8005 

        // addr: 0x8005 + 1 to reach break
        check(&mut cpu, expect![[""]])
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
        check(&mut cpu, expect![[""]])
    }
}