// src/gui/theme.rs
use super::settings::ThemeType;
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
    pub fn from_type(theme_type: ThemeType) -> Self {
        match theme_type {
            ThemeType::RetroNeon => Self::retro_neon(),
            ThemeType::TokyoNight => Self::tokyo_night(),
            ThemeType::Catppuccin => Self::catppuccin_mocha(),
            ThemeType::Gruvbox => Self::gruvbox_dark(),
            ThemeType::Kanagawa => Self::kanagawa(),
            ThemeType::SolarizedDark => Self::solarized_dark(),
            ThemeType::Dracula => Self::dracula(),
            ThemeType::Nord => Self::nord(),
            ThemeType::SolarizedLight => Self::solarized_light(),
            ThemeType::GruvboxLight => Self::gruvbox_light(),
            ThemeType::CatppuccinLatte => Self::catppuccin_latte(),
            ThemeType::TokyoDay => Self::tokyo_day(),
        }
    }

    // Dark Themes

    fn retro_neon() -> Self {
        Self {
            background: egui::Color32::from_rgb(10, 10, 15),
            surface: egui::Color32::from_rgb(20, 25, 35),
            surface_hover: egui::Color32::from_rgb(30, 40, 55),
            primary: egui::Color32::from_rgb(0, 255, 159),
            primary_hover: egui::Color32::from_rgb(57, 255, 182),
            text_primary: egui::Color32::from_rgb(255, 255, 255),
            text_secondary: egui::Color32::from_rgb(150, 200, 255),
            accent: egui::Color32::from_rgb(255, 100, 255),
            error: egui::Color32::from_rgb(255, 50, 100),
        }
    }

    fn tokyo_night() -> Self {
        Self {
            background: egui::Color32::from_rgb(26, 27, 38),
            surface: egui::Color32::from_rgb(36, 40, 59),
            surface_hover: egui::Color32::from_rgb(52, 59, 88),
            primary: egui::Color32::from_rgb(122, 162, 247),
            primary_hover: egui::Color32::from_rgb(158, 192, 255),
            text_primary: egui::Color32::from_rgb(192, 202, 245),
            text_secondary: egui::Color32::from_rgb(120, 138, 179),
            accent: egui::Color32::from_rgb(187, 154, 247),
            error: egui::Color32::from_rgb(247, 118, 142),
        }
    }

    fn catppuccin_mocha() -> Self {
        Self {
            background: egui::Color32::from_rgb(30, 30, 46),
            surface: egui::Color32::from_rgb(49, 50, 68),
            surface_hover: egui::Color32::from_rgb(69, 71, 90),
            primary: egui::Color32::from_rgb(137, 180, 250),
            primary_hover: egui::Color32::from_rgb(180, 210, 255),
            text_primary: egui::Color32::from_rgb(205, 214, 244),
            text_secondary: egui::Color32::from_rgb(166, 173, 200),
            accent: egui::Color32::from_rgb(245, 194, 231),
            error: egui::Color32::from_rgb(243, 139, 168),
        }
    }

    fn gruvbox_dark() -> Self {
        Self {
            background: egui::Color32::from_rgb(40, 40, 40),
            surface: egui::Color32::from_rgb(60, 56, 54),
            surface_hover: egui::Color32::from_rgb(80, 73, 69),
            primary: egui::Color32::from_rgb(184, 187, 38),
            primary_hover: egui::Color32::from_rgb(215, 153, 33),
            text_primary: egui::Color32::from_rgb(235, 219, 178),
            text_secondary: egui::Color32::from_rgb(168, 153, 132),
            accent: egui::Color32::from_rgb(250, 189, 47),
            error: egui::Color32::from_rgb(251, 73, 52),
        }
    }

    fn kanagawa() -> Self {
        Self {
            background: egui::Color32::from_rgb(31, 31, 40),
            surface: egui::Color32::from_rgb(42, 42, 51),
            surface_hover: egui::Color32::from_rgb(54, 54, 66),
            primary: egui::Color32::from_rgb(126, 156, 216),
            primary_hover: egui::Color32::from_rgb(149, 179, 239),
            text_primary: egui::Color32::from_rgb(220, 215, 186),
            text_secondary: egui::Color32::from_rgb(147, 153, 178),
            accent: egui::Color32::from_rgb(149, 127, 184),
            error: egui::Color32::from_rgb(195, 87, 87),
        }
    }

    fn solarized_dark() -> Self {
        Self {
            background: egui::Color32::from_rgb(0, 43, 54),
            surface: egui::Color32::from_rgb(7, 54, 66),
            surface_hover: egui::Color32::from_rgb(88, 110, 117),
            primary: egui::Color32::from_rgb(38, 139, 210),
            primary_hover: egui::Color32::from_rgb(42, 161, 152),
            text_primary: egui::Color32::from_rgb(131, 148, 150),
            text_secondary: egui::Color32::from_rgb(88, 110, 117),
            accent: egui::Color32::from_rgb(211, 54, 130),
            error: egui::Color32::from_rgb(220, 50, 47),
        }
    }

    fn dracula() -> Self {
        Self {
            background: egui::Color32::from_rgb(40, 42, 54),
            surface: egui::Color32::from_rgb(68, 71, 90),
            surface_hover: egui::Color32::from_rgb(98, 114, 164),
            primary: egui::Color32::from_rgb(139, 233, 253),
            primary_hover: egui::Color32::from_rgb(189, 147, 249),
            text_primary: egui::Color32::from_rgb(248, 248, 242),
            text_secondary: egui::Color32::from_rgb(98, 114, 164),
            accent: egui::Color32::from_rgb(255, 121, 198),
            error: egui::Color32::from_rgb(255, 85, 85),
        }
    }

    fn nord() -> Self {
        Self {
            background: egui::Color32::from_rgb(46, 52, 64),
            surface: egui::Color32::from_rgb(59, 66, 82),
            surface_hover: egui::Color32::from_rgb(67, 76, 94),
            primary: egui::Color32::from_rgb(136, 192, 208),
            primary_hover: egui::Color32::from_rgb(143, 188, 187),
            text_primary: egui::Color32::from_rgb(236, 239, 244),
            text_secondary: egui::Color32::from_rgb(216, 222, 233),
            accent: egui::Color32::from_rgb(180, 142, 173),
            error: egui::Color32::from_rgb(191, 97, 106),
        }
    }

    // Light Themes

    fn solarized_light() -> Self {
        Self {
            background: egui::Color32::from_rgb(253, 246, 227),
            surface: egui::Color32::from_rgb(238, 232, 213),
            surface_hover: egui::Color32::from_rgb(147, 161, 161),
            primary: egui::Color32::from_rgb(38, 139, 210),
            primary_hover: egui::Color32::from_rgb(42, 161, 152),
            text_primary: egui::Color32::from_rgb(101, 123, 131),
            text_secondary: egui::Color32::from_rgb(147, 161, 161),
            accent: egui::Color32::from_rgb(211, 54, 130),
            error: egui::Color32::from_rgb(220, 50, 47),
        }
    }

    fn gruvbox_light() -> Self {
        Self {
            background: egui::Color32::from_rgb(251, 241, 199),
            surface: egui::Color32::from_rgb(235, 219, 178),
            surface_hover: egui::Color32::from_rgb(213, 196, 161),
            primary: egui::Color32::from_rgb(121, 116, 14),
            primary_hover: egui::Color32::from_rgb(175, 58, 3),
            text_primary: egui::Color32::from_rgb(60, 56, 54),
            text_secondary: egui::Color32::from_rgb(102, 92, 84),
            accent: egui::Color32::from_rgb(181, 118, 20),
            error: egui::Color32::from_rgb(157, 0, 6),
        }
    }

    fn catppuccin_latte() -> Self {
        Self {
            background: egui::Color32::from_rgb(239, 241, 245),
            surface: egui::Color32::from_rgb(230, 233, 239),
            surface_hover: egui::Color32::from_rgb(204, 208, 218),
            primary: egui::Color32::from_rgb(30, 102, 245),
            primary_hover: egui::Color32::from_rgb(114, 135, 253),
            text_primary: egui::Color32::from_rgb(76, 79, 105),
            text_secondary: egui::Color32::from_rgb(140, 143, 161),
            accent: egui::Color32::from_rgb(234, 118, 203),
            error: egui::Color32::from_rgb(210, 15, 57),
        }
    }

    fn tokyo_day() -> Self {
        Self {
            background: egui::Color32::from_rgb(213, 214, 219),
            surface: egui::Color32::from_rgb(228, 229, 235),
            surface_hover: egui::Color32::from_rgb(200, 201, 210),
            primary: egui::Color32::from_rgb(52, 84, 138),
            primary_hover: egui::Color32::from_rgb(68, 109, 184),
            text_primary: egui::Color32::from_rgb(52, 59, 88),
            text_secondary: egui::Color32::from_rgb(120, 138, 179),
            accent: egui::Color32::from_rgb(114, 69, 139),
            error: egui::Color32::from_rgb(184, 52, 67),
        }
    }
}

pub fn apply_custom_style(ctx: &egui::Context, theme: &Theme) {
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
