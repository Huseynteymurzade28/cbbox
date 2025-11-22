// src/main.rs

mod audio;
mod constants;
mod emu;
mod gui; // GUI module

use std::env;
use std::fs::File;
use std::io::Read;

use audio::SquareWave;
use constants::*;
use emu::Emu; // Using the struct from audio.rs

use sdl2::audio::AudioSpecDesired;
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
    // 1. Parse Command Line Arguments or Show GUI
    let args: Vec<String> = env::args().collect();

    let rom_path = if args.len() >= 2 {
        // If a ROM path is provided as argument, use it directly
        args[1].clone()
    } else {
        // Otherwise, show GUI to select ROM
        match gui::show_rom_selector()? {
            Some(path) => path,
            None => {
                println!("No ROM selected. Exiting...");
                return Ok(());
            }
        }
    };

    // 2. Initialize SDL2 Subsystems
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?; // Initialize Audio

    let window = video_subsystem
        .window("Chippy - CHIP-8 Emulator v1.1", WINDOW_WIDTH, WINDOW_HEIGHT)
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

    // --- AUDIO SETUP START ---
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),  // CD Quality
        channels: Some(1),  // Mono
        samples: Some(256), // Default buffer size
    };

    // Open the audio device using the SquareWave struct from audio.rs
    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32, // 440Hz (A4 Note)
                phase: 0.0,
                volume: 0.15, // Low volume to be safe
            }
        })
        .map_err(|e| e.to_string())?;
    // --- AUDIO SETUP END ---

    // 3. Initialize Emulator & Load ROM
    let mut chip8 = Emu::new();

    let mut rom_file = File::open(&rom_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    rom_file
        .read_to_end(&mut buffer)
        .map_err(|e| e.to_string())?;

    chip8.load_rom(&buffer);
    println!("🚀 ROM Loaded: {}", &rom_path);

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

        // B. CPU Cycles
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }

        // C. Timers
        chip8.tick_timers();

        // --- AUDIO CONTROL ---
        // If the sound timer > 0, play sound. Otherwise, pause.
        if chip8.get_sound_timer() > 0 {
            device.resume();
        } else {
            device.pause();
        }

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

        // 60 FPS Delay
        ::std::thread::sleep(Duration::from_micros(16600));
    }

    Ok(())
}

// Keyboard Mapping Helper
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
