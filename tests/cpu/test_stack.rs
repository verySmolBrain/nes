#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;

    #[test]
    fn test_stack_push_u8() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.stack_push_u8(0x42);
        assert_eq!(0x42, cpu.stack_pop_u8());
    }

    #[test]
    fn test_stack_push_u16() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.stack_push_u16(0x4042);
        assert_eq!(0x4042, cpu.stack_pop_u16());
    }
}