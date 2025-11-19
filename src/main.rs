mod emu;
mod constants;

use std::env;
use std::fs::File;
use std::io::Read;

use emu::Emu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use constants::*;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <rom_file>");
        return Ok(());
    }
    let rom_path = &args[1];

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Chip-8 Emulator - Rust", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut chip8 = Emu::new();

    let mut rom_file = File::open(rom_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    rom_file
        .read_to_end(&mut buffer)
        .map_err(|e| e.to_string())?;

    chip8.load_rom(&buffer);
    println!("ROM loaded: {}", rom_path);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        chip8.tick();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let screen_buf = chip8.get_display();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for (i, pixel) in screen_buf.iter().enumerate() {
            if *pixel {
                let x = (i % SCREEN_WIDTH) as u32;
                let y = (i / SCREEN_WIDTH) as u32;

                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                canvas.fill_rect(rect)?;
            }
        }

        canvas.present();

        ::std::thread::sleep(Duration::from_millis(2));
    }

    Ok(())
}
