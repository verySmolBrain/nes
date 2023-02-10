#[cfg(test)]
mod test {
    use nes::emulator::joypad::Buttons;
    use nes::emulator::bus::Bus;
    use nes::emulator::memory::Mem;
    use crate::helpers::{ TestRom, load_into_memory, check };
    use expect_test::expect;

    #[test]
    fn test_joypad_read_correctly() {
        let mut bus = Bus::new(TestRom::default_rom());
        
        bus.joypad.press(Buttons::A);
        bus.mem_write(0x4016, 0x00);
        
        let button_1 = bus.mem_read(0x4016);
        println!("Button 1: {} {:08b}", button_1, bus.joypad.state.bits());
        let button_1 = bus.mem_read(0x4016);
        println!("Button 1: {} {:08b}", button_1, bus.joypad.state.bits());
    }
}