use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Buttons: u8 {
        const Right  = 0b1000_0000;
        const Left   = 0b0100_0000;
        const Down   = 0b0010_0000;
        const Up     = 0b0001_0000;
        const Start  = 0b0000_1000;
        const Select = 0b0000_0100;
        const B      = 0b0000_0010;
        const A      = 0b0000_0001;
    }
}

pub struct Joypad {
    pub strobe: bool,
    pub buttons: Box<dyn Iterator<Item = Buttons>>,
    pub state: Buttons,
    pub buffer: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            strobe: false,
            buttons: Box::from(Buttons::all().iter()),
            state: Buttons::empty(),
            buffer: 0,
        }
    }

    pub fn write(&mut self, value: u8) {
        self.strobe = value & 1 == 1;
        if self.strobe {
            self.buttons = Box::from(Buttons::all().iter());
        }
    }

    pub fn read(&mut self) -> u8 {
        let current_state = self.buffer;
        if !self.strobe {
            if let Some(button) = self.buttons.next() {
                self.buffer = button.bits() & self.state.bits();
            }   
        }
        current_state
    }

    pub fn press(&mut self, button: Buttons) {
        self.state.insert(button);
    }

    pub fn release(&mut self, button: Buttons) {
        self.state.remove(button);
    }

    pub fn next(&mut self) -> u8 {
        if let Some(button) = self.buttons.next() {
            button.bits() & self.state.bits()
        } else {
            1
        }
    }
}