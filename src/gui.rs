// src/gui.rs
use eframe::egui;
use std::fs;
use std::sync::{Arc, Mutex};

pub struct RomSelector {
    roms: Vec<RomEntry>,
    selected_rom: Arc<Mutex<Option<String>>>,
}

#[derive(Clone)]
struct RomEntry {
    name: String,
    path: String,
}

impl RomSelector {
    pub fn new(selected_rom: Arc<Mutex<Option<String>>>) -> Self {
        let roms = Self::scan_roms();
        Self { roms, selected_rom }
    }

    fn scan_roms() -> Vec<RomEntry> {
        let mut rom_list = Vec::new();

        if let Ok(entries) = fs::read_dir("assets") {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".ch8") {
                        rom_list.push(RomEntry {
                            name: filename.to_string(),
                            path: entry.path().to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }

        rom_list.sort_by(|a, b| a.name.cmp(&b.name));
        rom_list
    }
}

impl eframe::App for RomSelector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎮 CHIP-8 Emulator");
            ui.add_space(10.0);

            ui.label("Select a ROM to play:");
            ui.separator();
            ui.add_space(10.0);

            if self.roms.is_empty() {
                ui.colored_label(
                    egui::Color32::RED,
                    "⚠ No ROM files found in 'assets' folder!",
                );
                ui.label("Please add .ch8 files to the assets directory.");
            } else {
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        for rom in &self.roms {
                            if ui.button(&rom.name).clicked() {
                                *self.selected_rom.lock().unwrap() = Some(rom.path.clone());
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    });
            }

            ui.add_space(20.0);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("💡 Tip:");
                ui.small("You can also run: cargo run assets/game.ch8");
            });
        });
    }
}

pub fn show_rom_selector() -> Result<Option<String>, String> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_resizable(false),
        ..Default::default()
    };

    let selected_rom = Arc::new(Mutex::new(None));
    let selected_rom_clone = selected_rom.clone();

    eframe::run_native(
        "CHIP-8 ROM Selector",
        options,
        Box::new(move |_cc| Ok(Box::new(RomSelector::new(selected_rom_clone)))),
    )
    .map_err(|e| e.to_string())?;

    let result = selected_rom.lock().unwrap().clone();
    Ok(result)
}
