// src/gui/widgets.rs
use super::theme::Theme;
use eframe::egui;

#[derive(Clone)]
pub struct RomEntry {
    pub name: String,
    pub path: String,
}

impl RomEntry {
    pub fn new(name: String, path: String) -> Self {
        Self { name, path }
    }

    /// ROM kartı widget'ı - retro card tasarımı
    pub fn show_card(&self, ui: &mut egui::Ui, theme: &Theme) -> egui::Response {
        let desired_size = egui::vec2(ui.available_width(), 85.0); // Daha yüksek kartlar
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            // Hover efekti için renk seç
            let bg_color = if response.hovered() {
                theme.surface_hover
            } else {
                theme.surface
            }; // Card arka planı
            ui.painter()
                .rect_filled(rect, egui::Rounding::same(10.0), bg_color);

            // Sol tarafta NEON accent bar (daha kalın)
            let accent_rect = egui::Rect::from_min_size(rect.min, egui::vec2(6.0, rect.height()));
            ui.painter().rect_filled(
                accent_rect,
                egui::Rounding {
                    nw: 10.0,
                    sw: 10.0,
                    ne: 0.0,
                    se: 0.0,
                },
                theme.accent, // Neon pembe kullan
            );

            // Glow efekti için ikinci bir ince çizgi
            let glow_rect = egui::Rect::from_min_size(
                rect.min + egui::vec2(6.0, 0.0),
                egui::vec2(1.0, rect.height()),
            );
            ui.painter().rect_filled(
                glow_rect,
                egui::Rounding::ZERO,
                egui::Color32::from_rgba_unmultiplied(
                    theme.accent.r(),
                    theme.accent.g(),
                    theme.accent.b(),
                    100,
                ),
            );

            // Border hover efekti - neon glow
            if response.hovered() {
                ui.painter().rect_stroke(
                    rect,
                    egui::Rounding::same(10.0),
                    egui::Stroke::new(3.0, theme.primary), // Daha kalın neon yeşil
                );
                // İkinci glow katmanı
                ui.painter().rect_stroke(
                    rect.expand(1.5),
                    egui::Rounding::same(11.0),
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(
                            theme.primary.r(),
                            theme.primary.g(),
                            theme.primary.b(),
                            80,
                        ),
                    ),
                );
            }

            // İçerik
            let content_rect = rect.shrink2(egui::vec2(20.0, 15.0));
            let mut ui_content = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );

            // Icon - daha büyük
            ui_content.add_space(10.0);
            ui_content.label(egui::RichText::new("🎮").size(36.0));

            ui_content.add_space(18.0);

            // ROM ismi ve detaylar
            ui_content.vertical(|ui| {
                ui.label(
                    egui::RichText::new(&self.name)
                        .size(18.0) // Daha büyük font
                        .color(theme.primary) // Neon yeşil isim
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("▶ Click to play")
                        .size(13.0)
                        .color(theme.text_secondary),
                );
            });

            // Sağ tarafta neon ok işareti (hover'da görünür)
            if response.hovered() {
                ui_content.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("▶▶").size(24.0).color(theme.accent));
                });
            }
        }

        response
    }
}

/// Header bileşeni - RETRO STYLE
pub fn show_header(ui: &mut egui::Ui, theme: &Theme) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);

        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("🎮 CHIP-8 EMULATOR 🎮")
                .size(42.0)
                .color(theme.primary)
                .strong(),
        );

        ui.add_space(10.0);

        // Alt başlık - neon pembe
        ui.label(
            egui::RichText::new("⚡ SELECT A ROM TO START PLAYING ⚡")
                .size(16.0)
                .color(theme.accent)
                .strong(),
        );

        ui.add_space(12.0);

        // Retro scanline efekti
        let painter = ui.painter();
        let rect = ui.available_rect_before_wrap();
        let center_x = rect.center().x;
        let y = rect.top();

        // Neon çizgiler - multiple layers için glow efekti
        painter.line_segment(
            [
                egui::pos2(center_x - 250.0, y),
                egui::pos2(center_x + 250.0, y),
            ],
            egui::Stroke::new(3.0, theme.accent),
        );
        painter.line_segment(
            [
                egui::pos2(center_x - 250.0, y - 2.0),
                egui::pos2(center_x + 250.0, y - 2.0),
            ],
            egui::Stroke::new(1.0, theme.primary),
        );

        ui.add_space(15.0);
    });
}

/// Footer/bilgi paneli - RETRO
pub fn show_footer(ui: &mut egui::Ui, theme: &Theme, rom_count: usize) {
    ui.add_space(15.0);

    // Neon ayırıcı
    let painter = ui.painter();
    let rect = ui.available_rect_before_wrap();
    painter.line_segment(
        [rect.left_top(), rect.right_top()],
        egui::Stroke::new(2.0, theme.primary),
    );

    ui.add_space(15.0);

    let screen = ui.ctx().screen_rect(); // ekran boyutu

    let top_left = screen.min + egui::vec2(10.0, 10.0);

    ui.allocate_ui_at_rect(
        egui::Rect::from_min_size(top_left, egui::vec2(200.0, 30.0)),
        |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("{} ROMS LOADED", rom_count))
                        .color(theme.primary)
                        .size(14.0)
                        .strong(),
                );
            });
        },
    );
}

/// Boş durum ekranı (ROM bulunamadığında) - RETRO ERROR
pub fn show_empty_state(ui: &mut egui::Ui, theme: &Theme) {
    ui.vertical_centered(|ui| {
        ui.add_space(80.0);

        // Büyük hata ikonu
        ui.label(egui::RichText::new("⚠️").size(96.0));

        ui.add_space(25.0);

        // ERROR mesajı - neon kırmızı
        ui.label(
            egui::RichText::new("━━━ ERROR ━━━")
                .size(28.0)
                .color(theme.error)
                .strong(),
        );

        ui.add_space(15.0);

        ui.label(
            egui::RichText::new("NO ROM FILES FOUND")
                .size(24.0)
                .color(theme.error)
                .strong(),
        );

        ui.add_space(20.0);

        ui.label(
            egui::RichText::new("Add .ch8 files to the assets/ directory")
                .size(15.0)
                .color(theme.text_secondary),
        );

        ui.add_space(40.0);

        // Retro info box - neon yeşil border
        egui::Frame::none()
            .stroke(egui::Stroke::new(2.0, theme.primary))
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::same(20.0))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("💡 TIP")
                            .size(16.0)
                            .color(theme.accent)
                            .strong(),
                    );
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new("Download CHIP-8 ROMs from the internet")
                            .size(14.0)
                            .color(theme.text_secondary),
                    );
                });
            });
    });
}
