// crates/ui/src/theme/mod.rs

mod colors;
mod registry;
mod types;

pub use colors::ThemeColors;
pub use registry::{ActiveTheme, ThemeRegistry};
pub use types::*;

use gpui::{px, Pixels};

/// The main Theme struct used at runtime
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub mode: ThemeMode,
    pub colors: ThemeColors,
    pub radius: Radius,
    pub spacing: Spacing,
    pub typography: Typography,
}

#[derive(Debug, Clone)]
pub struct Radius {
    pub sm: Pixels,
    pub md: Pixels,
    pub lg: Pixels,
    pub full: Pixels,
}

impl From<RadiusConfig> for Radius {
    fn from(config: RadiusConfig) -> Self {
        Self {
            sm: px(config.sm),
            md: px(config.md),
            lg: px(config.lg),
            full: px(config.full),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Spacing {
    pub xs: Pixels,
    pub sm: Pixels,
    pub md: Pixels,
    pub lg: Pixels,
    pub xl: Pixels,
    pub xxl: Pixels,
}

impl From<SpacingConfig> for Spacing {
    fn from(config: SpacingConfig) -> Self {
        Self {
            xs: px(config.xs),
            sm: px(config.sm),
            md: px(config.md),
            lg: px(config.lg),
            xl: px(config.xl),
            xxl: px(config.xxl),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Typography {
    pub font_family: String,
    pub font_mono: String,
    pub size_xs: Pixels,
    pub size_sm: Pixels,
    pub size_base: Pixels,
    pub size_lg: Pixels,
    pub size_xl: Pixels,
    pub size_2xl: Pixels,
    pub weight_normal: u16,
    pub weight_medium: u16,
    pub weight_semibold: u16,
    pub weight_bold: u16,
}

impl From<TypographyConfig> for Typography {
    fn from(config: TypographyConfig) -> Self {
        Self {
            font_family: config.font_family,
            font_mono: config.font_mono,
            size_xs: px(config.size_xs),
            size_sm: px(config.size_sm),
            size_base: px(config.size_base),
            size_lg: px(config.size_lg),
            size_xl: px(config.size_xl),
            size_2xl: px(config.size_2xl),
            weight_normal: config.weight_normal,
            weight_medium: config.weight_medium,
            weight_semibold: config.weight_semibold,
            weight_bold: config.weight_bold,
        }
    }
}

impl Theme {
    pub fn from_config(config: ThemeConfig) -> Self {
        Self {
            name: config.name,
            mode: config.mode,
            colors: ThemeColors::from_config(&config.colors),
            radius: config.radius.into(),
            spacing: config.spacing.into(),
            typography: config.typography.into(),
        }
    }
}

/// Initialize theme system
pub fn init() {
    // Load default themes
    let light_theme = include_str!("../../themes/default-light.json");
    let dark_theme = include_str!("../../themes/default-dark.json");

    ThemeRegistry::register_from_json(light_theme).expect("Failed to load default light theme");
    ThemeRegistry::register_from_json(dark_theme).expect("Failed to load default dark theme");
}
