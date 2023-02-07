#[cfg(test)]

mod test {
    use nes::emulator::rom::Mirroring;
    use nes::emulator::ppu::Status;
    use crate::helpers::{ default_ppu };
    use expect_test::expect;

    #[test]
    fn test_read_status() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.status.insert(Status::VBLANK_STARTED);
        ppu.scroll.latch = true;
        ppu.address.latch = true;

        ppu.read_status();

        let expected = expect!["00000000 false false"];
        expected.assert_eq(&format!("{:08b} {} {}", ppu.status, ppu.scroll.latch, ppu.address.latch));
    }
}