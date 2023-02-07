#[cfg(test)]

mod test {
    use nes::emulator::rom::Mirroring;
    use crate::helpers::{ default_ppu };
    use expect_test::expect;

    #[test]
    fn test_vram_write_0x2000() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.write_address(0x20);
        ppu.write_address(0x00);

        ppu.write_data(0x01);

        let expected = expect![[r#"
            1
        "#]];
        expected.assert_debug_eq(&ppu.vram[0]);
    }

    #[test]
    fn test_vram_read_0x2000() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.write_address(0x20);
        ppu.write_address(0x00);

        ppu.write_data(0x01);

        ppu.write_address(0x20);
        ppu.write_address(0x00);

        ppu.read_data(); // Dummy read

        let expected = expect![[r#"
            1
        "#]];
        expected.assert_debug_eq(&ppu.read_data());
    }

    #[test]
    fn test_palette_read() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.write_address(0x3F);
        ppu.write_address(0x00);

        ppu.write_data(0x01);

        let expected = expect![[r#"
            1
        "#]];
        expected.assert_debug_eq(&ppu.palette_table[0]);
    }

    #[test]
    fn test_palette_read_mirroring() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.write_address(0x3F);
        ppu.write_address(0x10);

        ppu.write_data(0x01);

        let expected = expect![[r#"
            1
        "#]];
        expected.assert_debug_eq(&ppu.palette_table[0]);
    }
}