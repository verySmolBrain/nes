use nes::cpu::Cpu;
use nes::rom::Rom;
use nes::bus::Bus;
use nes::player::Player;
use std::env;

fn main() {
    let rom_path = env::args().nth(1).expect("No ROM path provided");

    let cartridge = Rom::load(&rom_path).unwrap();
    let bus = Bus::new(cartridge);
    let mut cpu = Cpu::new(bus);
    cpu.reset();

    let mut player = Player::new();
    player.run(cpu);
}