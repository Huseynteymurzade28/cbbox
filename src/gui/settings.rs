// src/gui/settings.rs
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeType {
    // Dark themes
    RetroNeon,
    TokyoNight,
    Catppuccin,
    Gruvbox,
    Kanagawa,
    SolarizedDark,
    Dracula,
    Nord,

    // Light themes
    SolarizedLight,
    GruvboxLight,
    CatppuccinLatte,
    TokyoDay,
}

impl ThemeType {
    pub fn all_themes() -> Vec<ThemeType> {
        vec![
            ThemeType::RetroNeon,
            ThemeType::TokyoNight,
            ThemeType::Catppuccin,
            ThemeType::Gruvbox,
            ThemeType::Kanagawa,
            ThemeType::SolarizedDark,
            ThemeType::Dracula,
            ThemeType::Nord,
            ThemeType::SolarizedLight,
            ThemeType::GruvboxLight,
            ThemeType::CatppuccinLatte,
            ThemeType::TokyoDay,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            ThemeType::RetroNeon => "Retro Neon",
            ThemeType::TokyoNight => "Tokyo Night",
            ThemeType::Catppuccin => "Catppuccin Mocha",
            ThemeType::Gruvbox => "Gruvbox Dark",
            ThemeType::Kanagawa => "Kanagawa",
            ThemeType::SolarizedDark => "Solarized Dark",
            ThemeType::Dracula => "Dracula",
            ThemeType::Nord => "Nord",
            ThemeType::SolarizedLight => "Solarized Light",
            ThemeType::GruvboxLight => "Gruvbox Light",
            ThemeType::CatppuccinLatte => "Catppuccin Latte",
            ThemeType::TokyoDay => "Tokyo Day",
        }
    }

    pub fn is_dark(&self) -> bool {
        !matches!(
            self,
            ThemeType::SolarizedLight
                | ThemeType::GruvboxLight
                | ThemeType::CatppuccinLatte
                | ThemeType::TokyoDay
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: ThemeType,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: ThemeType::RetroNeon,
            window_width: 800.0,
            window_height: 900.0,
        }
    }
}

impl Settings {
    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("chip8_emu");
        path.push("settings.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();

        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(settings) = serde_json::from_str(&contents) {
                return settings;
            }
        }

        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();

        // Config klasörünü oluştur
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, json).map_err(|e| e.to_string())?;

        Ok(())
    }
}
