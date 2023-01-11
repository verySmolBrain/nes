#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::memory::Mem;
    use crate::helpers::check_zero_and_negative;

    #[test]
    fn test_c6_zero_page_zero() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load(vec![0xc6, 0xa1, 0x00]);
        cpu.reset();

        cpu.mem_write(0xa1, 0x01);

        cpu.run();
        
        assert_eq!(cpu.mem_read(0xa1), 0x00);
        check_zero_and_negative(cpu, 0)
    }
}