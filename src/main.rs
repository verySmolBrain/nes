use nes::emulator::cpu::Cpu;
use nes::emulator::rom::Rom;
use nes::emulator::bus::Bus;
use nes::player::player::Player;
use std::env;

fn main() {
    // let rom_path = env::args().nth(1).expect("No ROM path provided");
    let rom_path = "rom/nestest.nes";
    // print all files and their contexts
    for entry in std::fs::read_dir("rom").unwrap() {
        let entry = entry.unwrap();
        println!("{:?}", entry.path());
    }

    let cartridge = Rom::load(&rom_path).unwrap();
    println!("Loaded cartridge");
    let bus = Bus::new(cartridge);
    println!("Loaded bus");
    let mut cpu = Cpu::new(bus);
    println!("Loaded cpu");
    cpu.reset();
    println!("Reset CPU");

    let mut player = Player::new(cpu);
    println!("Loaded player");
    player.run();
}