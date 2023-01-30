use nes::emulator::cpu::Cpu;
use nes::emulator::rom::Rom;
use nes::emulator::bus::Bus;
// use nes::player::Player;
use nes::helpers::trace::trace;
use std::env;

fn main() {
    let rom_path = env::args().nth(1).expect("No ROM path provided");

    let cartridge = Rom::load(&rom_path).unwrap();
    let bus = Bus::new(cartridge);
    let mut cpu = Cpu::new(bus);
    cpu.reset();

    // let mut player = Player::new();
    // player.run(cpu);
    cpu.program_counter = 0xc000;
    cpu.run_with_callback(|cpu|
        println!("{}", trace(cpu))
    )
}