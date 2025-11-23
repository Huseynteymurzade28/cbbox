// src/gui/app.rs
use eframe::egui;
use std::fs;
use std::sync::{Arc, Mutex};

use super::theme::{apply_custom_style, setup_custom_fonts, Theme};
use super::widgets::{show_empty_state, show_footer, show_header, RomEntry};

pub struct RomSelector {
    roms: Vec<RomEntry>,
    selected_rom: Arc<Mutex<Option<String>>>,
    theme: Theme,
}

impl RomSelector {
    pub fn new(cc: &eframe::CreationContext<'_>, selected_rom: Arc<Mutex<Option<String>>>) -> Self {
        // Custom fonts ve style ayarla
        setup_custom_fonts(&cc.egui_ctx);
        apply_custom_style(&cc.egui_ctx);

        let roms = Self::scan_roms();
        let theme = Theme::dark();

        Self {
            roms,
            selected_rom,
            theme,
        }
    }

    fn scan_roms() -> Vec<RomEntry> {
        let mut rom_list = Vec::new();

        if let Ok(entries) = fs::read_dir("assets") {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".ch8") {
                        rom_list.push(RomEntry::new(
                            filename.to_string(),
                            entry.path().to_string_lossy().to_string(),
                        ));
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
        // Her frame'de style'ı yeniden uygula (gerekirse)
        apply_custom_style(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            show_header(ui, &self.theme);

            ui.add_space(20.0);

            // Ana içerik alanı
            if self.roms.is_empty() {
                // Boş durum ekranı
                show_empty_state(ui, &self.theme);
            } else {
                // ROM listesi - scroll area ile
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add_space(10.0);

                        // Her ROM için card göster
                        for rom in &self.roms {
                            let response = rom.show_card(ui, &self.theme);

                            if response.clicked() {
                                *self.selected_rom.lock().unwrap() = Some(rom.path.clone());
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }

                            ui.add_space(8.0);
                        }

                        ui.add_space(10.0);
                    });
            }

            // Footer
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                show_footer(ui, &self.theme, self.roms.len());
            });
        });
    }
}

pub fn show_rom_selector() -> Result<Option<String>, String> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 900.0])  // Daha büyük pencere
            .with_min_inner_size([700.0, 800.0])
            .with_resizable(true)
            .with_title("🎮 CHIP-8 Retro Emulator")
            .with_transparent(false),
        centered: true,
        ..Default::default()
    };

    let selected_rom = Arc::new(Mutex::new(None));
    let selected_rom_clone = selected_rom.clone();

    eframe::run_native(
        "CHIP-8 Emulator",
        options,
        Box::new(move |cc| Ok(Box::new(RomSelector::new(cc, selected_rom_clone)))),
    )
    .map_err(|e| e.to_string())?;

    let result = selected_rom.lock().unwrap().clone();
    Ok(result)
}
