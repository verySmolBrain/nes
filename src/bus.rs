const BUS_ADDRESS_SPACE: usize = 0x800;

pub struct Bus {
    pub cpu_vram: [u8; BUS_ADDRESS_SPACE]
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_vram: [0; BUS_ADDRESS_SPACE]
        }
    }
}