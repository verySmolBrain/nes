use crate::cpu::Cpu;
use crate::memory::Mem;
use rand::Rng;
use sdl2::{
    event::Event,
    EventPump,
    keyboard::Keycode,
    pixels::Color,
    pixels::PixelFormatEnum,
    render::{ Canvas },
    video::{ Window },
};

pub struct Player {
    event_pump: EventPump,
    canvas: Canvas<Window>,
}

impl Player {
    pub fn new() -> Player {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("NES Emulator", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
            .position_centered()
            .build()
            .unwrap();
        
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        canvas.set_scale(10.0, 10.0).unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        return Player {
            event_pump,
            canvas,
        };
    }

    fn colour(&self, byte: u8) -> Color {
        match byte {
            0 => sdl2::pixels::Color::BLACK,
            1 => sdl2::pixels::Color::WHITE,
            2 | 9 => sdl2::pixels::Color::GREY,
            3 | 10 => sdl2::pixels::Color::RED,
            4 | 11 => sdl2::pixels::Color::GREEN,
            5 | 12 => sdl2::pixels::Color::BLUE,
            6 | 13 => sdl2::pixels::Color::MAGENTA,
            7 | 14 => sdl2::pixels::Color::YELLOW,
            _ => sdl2::pixels::Color::CYAN,
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

    fn read_screen_state(&mut self, cpu: &mut Cpu, frame: &mut [u8; 32 * 3 * 32]) -> bool {
        let mut frame_idx = 0;
        let mut update = false;
        for i in 0x0200..0x600 {
            let colour_idx = cpu.mem_read(i);
            let (b1, b2, b3) = self.colour(colour_idx).rgb();
            if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
                frame[frame_idx] = b1;
                frame[frame_idx + 1] = b2;
                frame[frame_idx + 2] = b3;
                update = true;
            }
            frame_idx += 3;
        }

        update
    }

    pub fn run(&mut self, mut cpu: Cpu) {
        let mut screen_state = [0 as u8; 32 * 3 * 32];
        let mut rng = rand::thread_rng();

        let creator = self.canvas.texture_creator();
        let mut texture = creator.create_texture_target(PixelFormatEnum::RGB24, 32, 32).unwrap();

        cpu.run_with_callback(move |cpu| {
            self.handle_user_input(cpu);
            cpu.mem_write(0xfe, rng.gen_range(1..16));
    
            if self.read_screen_state(cpu, &mut screen_state) {
                texture.update(None, &screen_state, 32 * 3).unwrap();
                self.canvas.copy(&texture, None, None).unwrap();
                self.canvas.present();
            }
    
            ::std::thread::sleep(std::time::Duration::new(0, 70_000));
        });
    }
}