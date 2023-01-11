#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::bus::Bus;
    use nes::cpu::Status;

   #[test]
   fn test_0xa9_lda_negative() {
       let bus = Bus::new();
       let mut cpu = CPU::new(bus);
       cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
       assert_eq!(cpu.register_a, 0xff);
       assert!(cpu.status.contains(Status::NEGATIVE));
       assert!(!cpu.status.contains(Status::ZERO));
   }
}