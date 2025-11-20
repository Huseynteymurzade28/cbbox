mod constants;
mod emu;

use std::env;
use std::fs::File;
use std::io::Read;

use constants::*;
use emu::Emu; // Import constants like SCREEN_WIDTH

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 10; // CPU speed multiplier

fn main() -> Result<(), String> {
    // 1. Parse Command Line Arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <rom_file>");
        return Ok(());
    }
    let rom_path = &args[1];

    // 2. Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Chippy - CHIP-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
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

    // 3. Initialize Emulator & Load ROM
    let mut chip8 = Emu::new();

    let mut rom_file = File::open(rom_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    rom_file
        .read_to_end(&mut buffer)
        .map_err(|e| e.to_string())?;

    chip8.load_rom(&buffer);
    println!("🚀 ROM Loaded: {}", rom_path);

    // 4. Main Game Loop
    'running: loop {
        // A. Input Handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        chip8.keypress(k, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        chip8.keypress(k, false);
                    }
                }
                _ => {}
            }
        }

        // B. CPU Instructions
        // Execute multiple CPU cycles for every single frame drawn (Speed up)
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }

        // C. Timers (Run at 60Hz)
        chip8.tick_timers();

        // D. Render
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

        // Sleep to maintain approx 60 FPS
        ::std::thread::sleep(Duration::from_micros(16600));
    }

    Ok(())
}

// Helper function to map Keyboard keys to CHIP-8 Keypad
// CHIP-8 Keypad:    Keyboard Mapping (QWERTY):
// 1 2 3 C           1 2 3 4
// 4 5 6 D    -->    Q W E R
// 7 8 9 E           A S D F
// A 0 B F           Z X C V
fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),

        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),

        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),

        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
