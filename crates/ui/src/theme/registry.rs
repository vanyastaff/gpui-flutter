// crates/ui/src/theme/registry.rs

use super::{Theme, ThemeConfig, ThemeMode};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

static THEME_REGISTRY: once_cell::sync::Lazy<Arc<RwLock<ThemeRegistryInner>>> =
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(ThemeRegistryInner::new())));

struct ThemeRegistryInner {
    themes: HashMap<String, Theme>,
    active_theme_name: String,
}

impl ThemeRegistryInner {
    fn new() -> Self {
        Self {
            themes: HashMap::new(),
            active_theme_name: String::new(),
        }
    }
}

pub struct ThemeRegistry;

impl ThemeRegistry {
    /// Register a theme from JSON string
    pub fn register_from_json(json: &str) -> anyhow::Result<()> {
        let config: ThemeConfig = serde_json::from_str(json)?;
        let theme = Theme::from_config(config);
        let name = theme.name.clone();

        let mut registry = THEME_REGISTRY.write().unwrap();
        registry.themes.insert(name.clone(), theme);

        // Set as active if first theme
        if registry.active_theme_name.is_empty() {
            registry.active_theme_name = name;
        }

        Ok(())
    }

    /// Set active theme by name
    pub fn set_active(name: &str) -> bool {
        let mut registry = THEME_REGISTRY.write().unwrap();
        if registry.themes.contains_key(name) {
            registry.active_theme_name = name.to_string();
            true
        } else {
            false
        }
    }

    /// Get active theme
    pub fn get_active() -> Theme {
        let registry = THEME_REGISTRY.read().unwrap();
        registry
            .themes
            .get(&registry.active_theme_name)
            .cloned()
            .unwrap_or_else(|| {
                // Fallback theme
                Self::create_fallback_theme()
            })
    }

    /// Get theme by name
    pub fn get(name: &str) -> Option<Theme> {
        let registry = THEME_REGISTRY.read().unwrap();
        registry.themes.get(name).cloned()
    }

    /// List all theme names
    pub fn list_themes() -> Vec<String> {
        let registry = THEME_REGISTRY.read().unwrap();
        registry.themes.keys().cloned().collect()
    }

    fn create_fallback_theme() -> Theme {
        // Minimal fallback theme
        let config = ThemeConfig {
            name: "fallback".to_string(),
            mode: ThemeMode::Light,
            colors: super::types::ThemeColorsConfig {
                background: "0 0% 100%".to_string(),
                foreground: "0 0% 0%".to_string(),
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
            },
            radius: Default::default(),
            spacing: Default::default(),
            typography: Default::default(),
        };

        Theme::from_config(config)
    }
}

/// Trait to get theme from context
pub trait ActiveTheme {
    fn theme(&self) -> Theme;
}

impl<T> ActiveTheme for T
where
    T: gpui::Context,
{
    fn theme(&self) -> Theme {
        ThemeRegistry::get_active()
    }
}
