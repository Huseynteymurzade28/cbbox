// src/gui/app.rs
use eframe::egui;
use std::fs;
use std::sync::{Arc, Mutex};

use super::settings::{Settings, ThemeType};
use super::theme::{apply_custom_style, setup_custom_fonts, Theme};
use super::widgets::{show_empty_state, show_footer, show_header, RomEntry};

pub struct RomSelector {
    roms: Vec<RomEntry>,
    selected_rom: Arc<Mutex<Option<String>>>,
    theme: Theme,
    settings: Settings,
    show_settings: bool,
}

impl RomSelector {
    pub fn new(cc: &eframe::CreationContext<'_>, selected_rom: Arc<Mutex<Option<String>>>) -> Self {
        // Ayarları yükle
        let settings = Settings::load();

        // Custom fonts ayarla
        setup_custom_fonts(&cc.egui_ctx);

        let roms = Self::scan_roms();
        let theme = Theme::from_type(settings.theme);

        // Style'ı tema ile uygula
        apply_custom_style(&cc.egui_ctx, &theme);

        Self {
            roms,
            selected_rom,
            theme,
            settings,
            show_settings: false,
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

    fn show_settings_panel(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .fixed_size([400.0, 500.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Theme Selection");
                ui.add_space(10.0);

                // Dark themes
                ui.label(
                    egui::RichText::new("Dark Themes")
                        .size(16.0)
                        .color(self.theme.primary)
                        .strong(),
                );
                ui.add_space(5.0);

                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for theme_type in ThemeType::all_themes() {
                            if theme_type.is_dark() {
                                let is_selected = self.settings.theme == theme_type;
                                let button = egui::Button::new(
                                    egui::RichText::new(theme_type.name()).size(14.0),
                                )
                                .min_size(egui::vec2(350.0, 35.0));

                                if ui.add(button).clicked() {
                                    self.settings.theme = theme_type;
                                    self.theme = Theme::from_type(theme_type);
                                    apply_custom_style(ctx, &self.theme);
                                    let _ = self.settings.save();
                                }

                                if is_selected {
                                    ui.label(
                                        egui::RichText::new("  ✓ Active").color(self.theme.primary),
                                    );
                                }
                                ui.add_space(5.0);
                            }
                        }

                        ui.add_space(15.0);
                        ui.label(
                            egui::RichText::new("Light Themes")
                                .size(16.0)
                                .color(self.theme.accent)
                                .strong(),
                        );
                        ui.add_space(5.0);

                        for theme_type in ThemeType::all_themes() {
                            if !theme_type.is_dark() {
                                let is_selected = self.settings.theme == theme_type;
                                let button = egui::Button::new(
                                    egui::RichText::new(theme_type.name()).size(14.0),
                                )
                                .min_size(egui::vec2(350.0, 35.0));

                                if ui.add(button).clicked() {
                                    self.settings.theme = theme_type;
                                    self.theme = Theme::from_type(theme_type);
                                    apply_custom_style(ctx, &self.theme);
                                    let _ = self.settings.save();
                                }

                                if is_selected {
                                    ui.label(
                                        egui::RichText::new("  ✓ Active").color(self.theme.primary),
                                    );
                                }
                                ui.add_space(5.0);
                            }
                        }
                    });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                if ui.button(egui::RichText::new("Close").size(14.0)).clicked() {
                    self.show_settings = false;
                }
            });
    }
}

impl eframe::App for RomSelector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Her frame'de style'ı yeniden uygula
        apply_custom_style(ctx, &self.theme);

        // Settings paneli
        if self.show_settings {
            self.show_settings_panel(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Settings butonu - sağ üst köşe
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                if ui.button(egui::RichText::new("Settings").size(24.0)).clicked() {
                    self.show_settings = !self.show_settings;
                }
            });

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
            .with_inner_size([800.0, 600.0]) // Daha büyük pencere
            .with_min_inner_size([300.0, 400.0])
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
