#[cfg(test)]
mod test {
    use nes::cpu::CPU;
    use nes::cpu::Status;

    #[test]
    fn test_0x78_sei() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x78, 0x00]);
        cpu.reset();
        assert!(cpu.status.contains(Status::INTERDIS)); // INTERDIS is initial true
        cpu.status.remove(Status::INTERDIS);
        assert!(!cpu.status.contains(Status::INTERDIS));
        cpu.run();
        assert!(cpu.status.contains(Status::INTERDIS));
    }
}