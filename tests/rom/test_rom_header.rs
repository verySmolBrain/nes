#[cfg(test)]

mod test {
    use nes::rom::Rom;
    use crate::helpers::{ TestRom };
    use expect_test::expect;

    const NES_HEADER: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

    #[test]
    fn test_invalid_nes_header() {
        let header = [0x4E, 0x45, 0x53, 0x1B];
        let num_prg_banks = 1;
        let num_chr_banks = 2;
        let flags_6 = 0b0000_0000;
        let flags_7 = 0b0000_0000;

        let rom = TestRom {
            nes_header: header,
            num_prg_rom: num_prg_banks,
            num_chr_rom: num_chr_banks,
            num_prg_ram: 0,
            flags_6,
            flags_7,
            trainer: None,
        };

        let cartridge = rom.test_rom_raw();
        let rom = Rom::new(cartridge);

        let expected = expect![[r#"
            Err(
                "Invalid NES Header",
            )
        "#]];
        expected.assert_debug_eq(&rom);
    }

    #[test]
    fn test_invalid_nes_version() {
        let header = NES_HEADER;
        let num_prg_banks = 1;
        let num_chr_banks = 2;
        let flags_6 = 0b0000_0000;
        let flags_7 = 0b0000_1100;

        let rom = TestRom {
            nes_header: header,
            num_prg_rom: num_prg_banks,
            num_chr_rom: num_chr_banks,
            num_prg_ram: 0,
            flags_6,
            flags_7,
            trainer: None,
        };

        let cartridge = rom.test_rom_raw();
        let rom = Rom::new(cartridge);

        let expected = expect![[r#"
            Err(
                "Unsupported NES Version",
            )
        "#]];
        expected.assert_debug_eq(&rom);
    }

    #[test]
    fn test_vertical_monitor() {
        let header = NES_HEADER;
        let num_prg_banks = 1;
        let num_chr_banks = 2;
        let flags_6 = 0b0000_0001;
        let flags_7 = 0b0000_0000;

        let rom = TestRom {
            nes_header: header,
            num_prg_rom: num_prg_banks,
            num_chr_rom: num_chr_banks,
            num_prg_ram: 0,
            flags_6,
            flags_7,
            trainer: None,
        };

        let cartridge = rom.test_rom_raw();
        let rom = Rom::new(cartridge).unwrap();

        let expected = expect![[r#"
            VERTICAL
        "#]];
        expected.assert_debug_eq(&rom.screen_mirroring);
    }
}