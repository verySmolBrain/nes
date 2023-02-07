#[cfg(test)]

mod test {
    use nes::emulator::rom::Mirroring;
    use nes::emulator::ppu::Controller;
    use crate::helpers::{ default_ppu };
    use expect_test::expect;

    #[test]
    fn test_write_address() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.write_address(0x20);
        ppu.write_address(0x30);

        let expected = expect!["0x2030"];
        expected.assert_eq(&format!("0x{:04X}", ppu.address.value()));
    }

    #[test]
    fn test_increment_address_controller_unset() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.write_address(0x20);
        ppu.write_address(0x00);

        ppu.address.next(Controller::empty());

        let expected = expect!["0x2001"];
        expected.assert_eq(&format!("0x{:04X}", ppu.address.value()));
    }

    #[test]
    fn test_increment_address_controller_unset_overflow() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.write_address(0x20);
        ppu.write_address(0xFF);

        ppu.address.next(Controller::empty());

        let expected = expect!["0x2100"];
        expected.assert_eq(&format!("0x{:04X}", ppu.address.value()));
    }

    #[test]
    fn test_increment_address_controller_set() {
        let mut ppu = default_ppu(Mirroring::HORIZONTAL);

        ppu.write_address(0x10);
        ppu.write_address(0x00);

        ppu.address.next(Controller::VRAM_INCREMENT);

        let expected = expect!["0x1020"];
        expected.assert_eq(&format!("0x{:04X}", ppu.address.value()));
    }
}