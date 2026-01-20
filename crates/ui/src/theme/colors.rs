// crates/ui/src/theme/colors.rs

use crate::color::Color;

#[derive(Debug, Clone)]
pub struct ThemeColors {
    // Base
    pub background: Color,
    pub foreground: Color,

    // Primary
    pub primary: Color,
    pub primary_foreground: Color,

    // Secondary
    pub secondary: Color,
    pub secondary_foreground: Color,

    // Muted
    pub muted: Color,
    pub muted_foreground: Color,

    // Accent
    pub accent: Color,
    pub accent_foreground: Color,

    // Destructive
    pub destructive: Color,
    pub destructive_foreground: Color,

    // Borders
    pub border: Color,
    pub input: Color,
    pub ring: Color,
}

impl ThemeColors {
    pub fn from_config(config: &super::types::ThemeColorsConfig) -> Self {
        Self {
            background: Color::parse_hsl_compact(&config.background),
            foreground: Color::parse_hsl_compact(&config.foreground),
            primary: Color::parse_hsl_compact(&config.primary),
            primary_foreground: Color::parse_hsl_compact(&config.primary_foreground),
            secondary: Color::parse_hsl_compact(&config.secondary),
            secondary_foreground: Color::parse_hsl_compact(&config.secondary_foreground),
            muted: Color::parse_hsl_compact(&config.muted),
            muted_foreground: Color::parse_hsl_compact(&config.muted_foreground),
            accent: Color::parse_hsl_compact(&config.accent),
            accent_foreground: Color::parse_hsl_compact(&config.accent_foreground),
            destructive: Color::parse_hsl_compact(&config.destructive),
            destructive_foreground: Color::parse_hsl_compact(&config.destructive_foreground),
            border: Color::parse_hsl_compact(&config.border),
            input: Color::parse_hsl_compact(&config.input),
            ring: Color::parse_hsl_compact(&config.ring),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_colors_from_config() {
        let config = super::super::types::ThemeColorsConfig {
            background: "0 0% 100%".to_string(),
            foreground: "222 84% 5%".to_string(),
            primary: "222 47% 11%".to_string(),
            primary_foreground: "210 40% 98%".to_string(),
            secondary: "210 40% 96%".to_string(),
            secondary_foreground: "222 47% 11%".to_string(),
            muted: "210 40% 96%".to_string(),
            muted_foreground: "215 16% 47%".to_string(),
            accent: "210 40% 96%".to_string(),
            accent_foreground: "222 47% 11%".to_string(),
            destructive: "0 84% 60%".to_string(),
            destructive_foreground: "210 40% 98%".to_string(),
            border: "214 32% 91%".to_string(),
            input: "214 32% 91%".to_string(),
            ring: "222 84% 5%".to_string(),
        };

        let colors = ThemeColors::from_config(&config);

        // Verify background is white (100% lightness)
        let bg_hsla = colors.background.to_hsla();
        assert!((bg_hsla.l - 1.0).abs() < 0.01);
    }
}
