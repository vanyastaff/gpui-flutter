// crates/ui/src/theme/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub mode: ThemeMode,
    pub colors: ThemeColorsConfig,
    pub radius: RadiusConfig,
    pub spacing: SpacingConfig,
    pub typography: TypographyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColorsConfig {
    // Base colors (HSL format: "hue saturation% lightness%")
    pub background: String,
    pub foreground: String,

    // Primary
    pub primary: String,
    pub primary_foreground: String,

    // Secondary
    pub secondary: String,
    pub secondary_foreground: String,

    // Muted
    pub muted: String,
    pub muted_foreground: String,

    // Accent
    pub accent: String,
    pub accent_foreground: String,

    // Destructive
    pub destructive: String,
    pub destructive_foreground: String,

    // Borders and inputs
    pub border: String,
    pub input: String,
    pub ring: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiusConfig {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub full: f32,
}

impl Default for RadiusConfig {
    fn default() -> Self {
        Self {
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            full: 9999.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingConfig {
    pub xs: f32,  // 4px
    pub sm: f32,  // 8px
    pub md: f32,  // 16px
    pub lg: f32,  // 24px
    pub xl: f32,  // 32px
    pub xxl: f32, // 48px
}

impl Default for SpacingConfig {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
            xxl: 48.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyConfig {
    pub font_family: String,
    pub font_mono: String,

    // Font sizes
    pub size_xs: f32,
    pub size_sm: f32,
    pub size_base: f32,
    pub size_lg: f32,
    pub size_xl: f32,
    pub size_2xl: f32,

    // Font weights
    pub weight_normal: u16,
    pub weight_medium: u16,
    pub weight_semibold: u16,
    pub weight_bold: u16,
}

impl Default for TypographyConfig {
    fn default() -> Self {
        Self {
            font_family: "system-ui".to_string(),
            font_mono: "monospace".to_string(),
            size_xs: 12.0,
            size_sm: 14.0,
            size_base: 16.0,
            size_lg: 18.0,
            size_xl: 20.0,
            size_2xl: 24.0,
            weight_normal: 400,
            weight_medium: 500,
            weight_semibold: 600,
            weight_bold: 700,
        }
    }
}
