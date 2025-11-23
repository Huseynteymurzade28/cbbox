// src/gui/theme.rs
use eframe::egui;

// Modern color palette
pub struct Theme {
    pub background: egui::Color32,
    pub surface: egui::Color32,
    pub surface_hover: egui::Color32,
    pub primary: egui::Color32,
    pub primary_hover: egui::Color32,
    pub text_primary: egui::Color32,
    pub text_secondary: egui::Color32,
    pub accent: egui::Color32,
    pub error: egui::Color32,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: egui::Color32::from_rgb(10, 10, 15), // Daha koyu retro arka plan
            surface: egui::Color32::from_rgb(20, 25, 35),    // Card arka planı (mavi ton)
            surface_hover: egui::Color32::from_rgb(30, 40, 55), // Hover efekti
            primary: egui::Color32::from_rgb(0, 255, 159),   // Neon yeşil (retro!)
            primary_hover: egui::Color32::from_rgb(57, 255, 182), // Açık neon yeşil
            text_primary: egui::Color32::from_rgb(255, 255, 255), // Beyaz
            text_secondary: egui::Color32::from_rgb(150, 200, 255), // Açık mavi
            accent: egui::Color32::from_rgb(255, 100, 255),  // Neon pembe/magenta
            error: egui::Color32::from_rgb(255, 50, 100),    // Neon kırmızı
        }
    }
}

pub fn apply_custom_style(ctx: &egui::Context) {
    let theme = Theme::dark();

    let mut style = (*ctx.style()).clone();

    // Genel renkler
    style.visuals.window_fill = theme.background;
    style.visuals.panel_fill = theme.background;
    style.visuals.extreme_bg_color = theme.surface;

    // Text renkleri
    style.visuals.override_text_color = Some(theme.text_primary);
    style.visuals.widgets.noninteractive.fg_stroke.color = theme.text_secondary;

    // Button stilleri
    style.visuals.widgets.inactive.weak_bg_fill = theme.surface;
    style.visuals.widgets.inactive.bg_stroke.color = theme.surface;
    style.visuals.widgets.hovered.weak_bg_fill = theme.surface_hover;
    style.visuals.widgets.hovered.bg_stroke.color = theme.primary;
    style.visuals.widgets.active.weak_bg_fill = theme.primary;

    // Spacing ve boyutlar
    style.spacing.item_spacing = egui::vec2(12.0, 8.0);
    style.spacing.button_padding = egui::vec2(16.0, 12.0);
    style.spacing.window_margin = egui::Margin::same(20.0);

    // Köşe yuvarlaklığı
    style.visuals.window_rounding = egui::Rounding::same(12.0);
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(8.0);

    ctx.set_style(style);
}

pub fn heading_font() -> egui::TextStyle {
    egui::TextStyle::Name("Heading".into())
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Font boyutları
    fonts.families.insert(
        egui::FontFamily::Proportional,
        vec!["Ubuntu-Light".to_owned(), "NotoEmoji-Regular".to_owned()],
    );

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            egui::TextStyle::Small,
            egui::FontId::new(12.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Body,
            egui::FontId::new(15.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Button,
            egui::FontId::new(15.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Monospace,
            egui::FontId::new(14.0, egui::FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Name("Heading".into()),
            egui::FontId::new(32.0, egui::FontFamily::Proportional),
        ),
    ]
    .into();

    ctx.set_fonts(fonts);
    ctx.set_style(style);
}
