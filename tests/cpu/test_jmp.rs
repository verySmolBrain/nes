#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::memory::Mem;
   
    #[test]
    fn test_0x4c_jmp_absolute() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x4c, 0x05, 0x80, 0x00]); // addr: 0x8005 
        cpu.reset();

        cpu.run();
        assert_eq!(cpu.program_counter, 0x8006); // addr: 0x8005 + 1 to reach break
    }

    #[test]
    fn test_0x6c_jmp_indirect_ff_bug() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x6c, 0xFF, 0x90, 0x00]); // addr: 0x80FF
        cpu.reset();

        // LSB: 0x80FF
        // MSB: 0x6c00
        cpu.mem_write_u16(0x90FF, 0x7776); // addr: 0x7677

        cpu.run();

        assert_eq!(cpu.program_counter, 0x7777); // addr: 0x7677 + 1 to reach break
    }
}