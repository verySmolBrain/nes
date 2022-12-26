#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;

    #[test]
    fn test_0x85_sta_zero_page() {
        let mut cpu = CPU::new();
        // Set carry flag to 1
        cpu.load(vec![0xf8, 0x00]);
        cpu.reset();
        assert!(cpu.status.contains(Status::INTERDIS)); // INTERDIS is initial true
        cpu.status.remove(Status::INTERDIS);
        assert!(!cpu.status.contains(Status::INTERDIS));
        cpu.run();
        assert!(cpu.status.contains(Status::DECIMAL));
    }
}