#[cfg(test)]

mod test {
    use nes::emulator::rom::Mirroring;
    use crate::helpers::{ default_ppu };
    use expect_test::expect;

    #[test]
    fn test_read_status_reset() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.write_scroll(0x03);
        ppu.write_scroll(0x07);

        let expected = expect!["3 7"];
        expected.assert_eq(&format!("{} {}", ppu.scroll.x, ppu.scroll.y));
    }
}