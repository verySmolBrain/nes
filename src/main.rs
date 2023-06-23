use nes::emulator::cpu::Cpu;
use nes::emulator::rom::Rom;
use nes::emulator::bus::Bus;
use nes::player::player::Player;
// use std::env;

fn main() {
    // let rom_path = env::args().nth(1).expect("No ROM path provided");
    let rom_path = "rom/pm.nes";

    let cartridge = Rom::load(&rom_path).unwrap();
    let bus = Bus::new(cartridge);

    let mut cpu = Cpu::new(bus);
    cpu.reset();

    let mut player = Player::new(cpu);
    let mut main_loop = move || {
        player.run();
    };

    #[cfg(target_os = "emscripten")]
    use nes::emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}