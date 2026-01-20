// crates/ui/src/theme/colors.rs

use gpui::{hsla, Hsla};

/// Parse HSL string "hue saturation% lightness%" to Hsla
pub fn parse_hsl(hsl_str: &str) -> Hsla {
    let parts: Vec<&str> = hsl_str.split_whitespace().collect();

    if parts.len() != 3 {
        eprintln!("Invalid HSL format: {}, using fallback", hsl_str);
        return hsla(0.0, 0.0, 0.0, 1.0);
    }

    let hue: f32 = parts[0].parse().unwrap_or(0.0) / 360.0;
    let saturation: f32 = parts[1].trim_end_matches('%').parse().unwrap_or(0.0) / 100.0;
    let lightness: f32 = parts[2].trim_end_matches('%').parse().unwrap_or(0.0) / 100.0;

    hsla(hue, saturation, lightness, 1.0)
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    // Base
    pub background: Hsla,
    pub foreground: Hsla,

    // Primary
    pub primary: Hsla,
    pub primary_foreground: Hsla,

    // Secondary
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,

    // Muted
    pub muted: Hsla,
    pub muted_foreground: Hsla,

    // Accent
    pub accent: Hsla,
    pub accent_foreground: Hsla,

    // Destructive
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,

    // Borders
    pub border: Hsla,
    pub input: Hsla,
    pub ring: Hsla,
}

impl ThemeColors {
    pub fn from_config(config: &super::types::ThemeColorsConfig) -> Self {
        Self {
            background: parse_hsl(&config.background),
            foreground: parse_hsl(&config.foreground),
            primary: parse_hsl(&config.primary),
            primary_foreground: parse_hsl(&config.primary_foreground),
            secondary: parse_hsl(&config.secondary),
            secondary_foreground: parse_hsl(&config.secondary_foreground),
            muted: parse_hsl(&config.muted),
            muted_foreground: parse_hsl(&config.muted_foreground),
            accent: parse_hsl(&config.accent),
            accent_foreground: parse_hsl(&config.accent_foreground),
            destructive: parse_hsl(&config.destructive),
            destructive_foreground: parse_hsl(&config.destructive_foreground),
            border: parse_hsl(&config.border),
            input: parse_hsl(&config.input),
            ring: parse_hsl(&config.ring),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hsl() {
        let color = parse_hsl("222 47% 11%");
        // hue: 222/360 = 0.617
        // sat: 47% = 0.47
        // light: 11% = 0.11
        assert!((color.h - 0.617).abs() < 0.01);
        assert!((color.s - 0.47).abs() < 0.01);
        assert!((color.l - 0.11).abs() < 0.01);
        assert_eq!(color.a, 1.0);
    }
}
