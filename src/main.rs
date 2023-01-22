use nes::cpu::Cpu;
use nes::rom::Rom;
use nes::player::Player;
use std::env;

fn main() {
    let rom_path = env::args().nth(1).expect("No ROM path provided");

    let cartridge = Rom::load(&rom_path).unwrap();
    let mut cpu = Cpu::new(cartridge);
    cpu.reset();

    let mut player = Player::new();
    player.run(cpu);
}