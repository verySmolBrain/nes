#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;

   #[test]
   fn test_0xa2_ldx_immediate_load_data() {
       let bus = Bus::new();
       let mut cpu = CPU::new(bus);
       cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
       assert_eq!(cpu.register_x, 0x05);
       assert!(!cpu.status.contains(Status::NEGATIVE));
       assert!(!cpu.status.contains(Status::ZERO));
   }
   
    #[test]
    fn test_0xa2_lda_zero_flag() {
        let bus = Bus::new();
        let mut cpu = CPU::new(bus);
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.status.contains(Status::ZERO));
    }
}