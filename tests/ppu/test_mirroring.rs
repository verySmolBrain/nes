#[cfg(test)]

mod test {
    use nes::emulator::rom::Mirroring;
    use nes::emulator::ppu::Controller;
    use crate::helpers::{ default_ppu };
    use expect_test::expect;

    #[test]
    fn test_horizontal_mirroring() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.write_address(0x20);
        ppu.write_address(0x00);

        ppu.write_data(0x01);

        ppu.write_address(0x24);
        ppu.write_address(0x01);

        ppu.write_data(0x02);

        ppu.write_address(0x28);
        ppu.write_address(0x00);

        ppu.write_data(0x01);

        ppu.write_address(0x2C);
        ppu.write_address(0x01);

        ppu.write_data(0x02);

        let expected = expect![[r#"
            1 2 
            1 2"#]];
        expected.assert_eq(&format!("{} {} \n{} {}", ppu.vram[0], ppu.vram[1], ppu.vram[0x400], ppu.vram[0x401]))
    }

    #[test]
    fn test_vertical_mirroring() {
        let mut ppu = default_ppu(Mirroring::VERTICAL);

        ppu.write_address(0x20);
        ppu.write_address(0x00);

        ppu.write_data(0x01);

        ppu.write_address(0x24);
        ppu.write_address(0x00);

        ppu.write_data(0x02);

        ppu.write_address(0x28);
        ppu.write_address(0x01);

        ppu.write_data(0x01);

        ppu.write_address(0x2C);
        ppu.write_address(0x01);

        ppu.write_data(0x02);

        let expected = expect![[r#"
            1 1 
            2 2"#]];
        expected.assert_eq(&format!("{} {} \n{} {}", ppu.vram[0], ppu.vram[1], ppu.vram[0x400], ppu.vram[0x401]))
    }
}