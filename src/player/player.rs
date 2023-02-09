use crate::emulator::ppu::{Ppu, Controller};
use crate::emulator::cpu::Cpu;
use crate::emulator::memory::Mem;
use crate::helpers::trace::trace;
use crate::player::palette;
use sdl2::render::Texture;
use sdl2::{
    event::Event,
    EventPump,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    render::Canvas,
    video::Window,
};

pub struct Frame {
    pub data: Vec<u8>,
}
 
impl Frame {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 240;
    const RGB_DATA_LEN: usize = 3;
 
    pub fn new() -> Self {
        Frame {
            data: vec![0; Frame::WIDTH * Frame::HEIGHT * Frame::RGB_DATA_LEN],
        }
    }
 
    pub fn update_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let base = (y * Frame::RGB_DATA_LEN) * Frame::WIDTH + (x * Frame::RGB_DATA_LEN);
        if base + 2 < self.data.len() {
            self.data[base] = rgb.0;
            self.data[base + 1] = rgb.1;
            self.data[base + 2] = rgb.2;
        }
    }
}

pub struct Player {
    event_pump: EventPump,
    canvas: Canvas<Window>,
}

impl Player {
    pub fn new() -> Player {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("NES Emulator", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
            .position_centered()
            .build()
            .unwrap();
        
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        canvas.set_scale(3.0, 3.0).unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        return Player {
            event_pump,
            canvas,
        };
    }

    pub fn render(&mut self, ppu: &Ppu, frame: &mut Frame, texture: &mut Texture) {
        let bank = if !ppu.controller.contains(Controller::BACKGROUND) { 0 } else { 1 };
        const FIRST_TABLE_END: usize = 0x03c0;

        for i in 0..FIRST_TABLE_END {
            let tile_n = ppu.vram[i];
            self.render_tile(&ppu.chr_rom, bank, tile_n as usize, frame, i % 32, i / 32);
        }

        texture.update(None, &frame.data, Frame::WIDTH * Frame::RGB_DATA_LEN).unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }

    fn render_tile(&self, chr_rom: &Vec<u8>, bank: usize, tile_n: usize, frame: &mut Frame, x: usize, y: usize) {
        const LEFT_BANK_START: usize = 0x0000;
        const RIGHT_BANK_START: usize = 0x1000;
        const TILE_LEN: usize = 16;

        let bank_start = if bank == 0 { LEFT_BANK_START } else { RIGHT_BANK_START };
        let tile_start = bank_start + tile_n * TILE_LEN;

        let tile = &chr_rom[
            tile_start .. tile_start + TILE_LEN
        ];
     
        for i in 0..8 {
            let mut upper = tile[i];
            let mut lower = tile[i + 8];
     
            for j in (0..8).rev() { // Initial frame is right -> left + LE
                let rgb = match (upper & 1 == 1, lower & 1 == 1) {
                    (false, false) => palette::DEFAULT_PALLETE[0x01],
                    (true, false) => palette::DEFAULT_PALLETE[0x23],
                    (false, true) => palette::DEFAULT_PALLETE[0x27],
                    (true, true) => palette::DEFAULT_PALLETE[0x30],
                };
                upper >>= 1;
                lower >>= 1;
                frame.update_pixel(x * 8 + j, y * 8 + i, rgb)
            }
        }
    }

    fn handle_user_input(&mut self, cpu: &mut Cpu) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(0);
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    cpu.mem_write(0xff, 0x77);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    cpu.mem_write(0xff, 0x73);
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    cpu.mem_write(0xff, 0x61);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    cpu.mem_write(0xff, 0x64);
                }
                _ => {}
            }
        }
    }

    pub fn run(&mut self, mut cpu: Cpu) {
        let mut frame = Frame::new();
        let creator = self.canvas.texture_creator();
        let mut texture = creator.create_texture_target(PixelFormatEnum::RGB24, 256, 240).unwrap();

        cpu.run_with_callback(move |cpu| {
            println!("{}", trace(cpu));

            let ppu = cpu.ppu_ready();
            if ppu.is_some() {
                self.render(ppu.unwrap(), &mut frame, &mut texture);
            }
            
            self.handle_user_input(cpu);
        })
    }
}