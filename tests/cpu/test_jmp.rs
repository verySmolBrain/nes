#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::memory::Mem;
   
    #[test]
    fn test_0x4c_jmp_absolute() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x4c, 0x05, 0x00, 0x00]); // addr: 0x8005 
        cpu.reset();

        cpu.run();
        assert_eq!(cpu.program_counter, 6); // addr: 0x8005 + 1 to reach break
    }

    #[test]
    fn test_0x6c_jmp_indirect_ff_bug() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0x6c, 0xFF, 0x00, 0x00]); // addr: 0x80FF
        cpu.reset();

        /*
        An original 6502 has does not correctly fetch the target address if the 
        indirect vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF). 
        In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00.
        https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
        */

        // LSB: 0x80FF
        // MSB: 0x6c00
        cpu.mem_write(0x00FF, 0x10);
        cpu.mem_write(0x0000, 0x11);

        cpu.run();

        assert_eq!(cpu.program_counter, 0x1111); // addr: 0x7677 + 1 to reach break
    }
}