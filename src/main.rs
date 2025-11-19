mod emu; // emu.rs dosyasını modül olarak ekle

use emu::Emu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const SCALE: u32 = 15; // Pikselleri 15 kat büyüt
const WINDOW_WIDTH: u32 = (emu::SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (emu::SCREEN_HEIGHT as u32) * SCALE;

fn main() -> Result<(), String> {
    // 1. SDL2 Başlatma
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
        .present_vsync() // Görüntü yırtılmasını önle
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    
    // 2. Emülatörü oluştur
    let mut chip8 = Emu::new();
    

    // 3. Ana Döngü (Game Loop)
    'running: loop {
        // A. Event Handling (Klavye vs.)
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // B. CPU Cycle
        chip8.tick(); // Şimdilik boş, işlem yapmıyor

        // C. Render (Çizim)
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear(); // Ekranı siyaha boya

        let screen_buf = chip8.get_display();

        canvas.set_draw_color(Color::RGB(255, 255, 255)); // Beyaz renk
        
        // Chip-8 buffer'ını gez ve 'true' olanları ekrana çiz
        for (i, pixel) in screen_buf.iter().enumerate() {
            if *pixel {
                // 1D array'den 2D koordinata dönüşüm:
                let x = (i % emu::SCREEN_WIDTH) as u32;
                let y = (i / emu::SCREEN_WIDTH) as u32;

                // Dikdörtgen çiz (SCALE ile büyütülmüş)
                let rect = Rect::new(
                    (x * SCALE) as i32, 
                    (y * SCALE) as i32, 
                    SCALE, 
                    SCALE
                );
                canvas.fill_rect(rect)?;
            }
        }

        canvas.present(); // Çizimi ekrana bas
        
        // Çok hızlı çalışmaması için minik bir delay (CPU hızı ayarı buraya gelecek)
        ::std::thread::sleep(Duration::from_millis(2));
    }

    Ok(())
}