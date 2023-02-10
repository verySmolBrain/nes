use std::collections::HashMap;

use sdl2::keyboard::Keycode;
use lazy_static::lazy_static;
use crate::emulator::joypad::Buttons;

lazy_static! { /* phf map doesn't support enums as keys */
    pub static ref CONTROLS_MAP: Vec<(Keycode, Buttons)> = vec![
        (Keycode::Down, Buttons::A),
        (Keycode::X, Buttons::B),
        (Keycode::Return, Buttons::Start),
        (Keycode::Backspace, Buttons::Select),
        (Keycode::Up, Buttons::Up),
        (Keycode::Down, Buttons::Down),
        (Keycode::Left, Buttons::Left),
        (Keycode::Right, Buttons::Right),
    ];

    pub static ref CONTROLS: HashMap<Keycode, Buttons> = {
        let mut map = HashMap::new();
        for (key, button) in CONTROLS_MAP.iter() {
            map.insert(*key, *button);
        }
        map
    };
}