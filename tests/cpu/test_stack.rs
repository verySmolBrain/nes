#[cfg(test)]
mod test {
    use nes::cpu::CPU;

    #[test]
    fn test_stack_push_u8() {
        let mut cpu = CPU::new();
        cpu.stack_push_u8(0x42);
        assert_eq!(0x42, cpu.stack_pop_u8());
    }

    #[test]
    fn test_stack_push_u16() {
        let mut cpu = CPU::new();
        cpu.stack_push_u16(0x4042);
        assert_eq!(0x4042, cpu.stack_pop_u16());
    }
}