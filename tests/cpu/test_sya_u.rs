#[cfg(test)]
mod test {
    use nes::emulator::cpu::Cpu;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_0x9c_sya_u_zero_page_load() {
        let mut bus = Bus::new(TestRom::default_rom());
        load_into_memory(&mut bus, vec![0x9c, 0x00, 0b0000_0011, 0x00], 0x0000);

        let mut cpu = Cpu::new(bus);
        cpu.program_counter = 0x0000;
        cpu.register_y = 0b1111_1111;
        
        check(&mut cpu, expect![[r#"
            0000  9C 00 03  SYA_U $0300,X @ 0300 = 00       A:00 X:00 Y:FF P:24 SP:FD PPU:  0,  0 CYC:0
            0003  00        BRK                             A:00 X:00 Y:FF P:24 SP:FD PPU:  0, 15 CYC:5"#]]);
        
        let expected = expect!["00000100"];
        expected.assert_eq(format!("{:08b}", cpu.mem_read(0b11_0000_0000)).as_str());
    }
}