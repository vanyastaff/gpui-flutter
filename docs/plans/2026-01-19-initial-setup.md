# gpui-flutter Initial Setup & Core Components Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Set up the gpui-flutter project structure with theme system, basic utilities, and first components (Container, Row, Column, Text, Button).

**Architecture:** 
- Workspace with two crates: `ui` (core library) and `gallery` (storybook)
- JSON-based theme system with semantic color tokens (shadcn/ui style)
- Flutter-style API with named parameters instead of nested `.child()` calls
- Built-in accessibility with ARIA support

**Tech Stack:**
- GPUI 0.2+ (GPU-accelerated UI framework)
- lucide-icons 0.555 (icon system)
- gpui-storybook 0.5 (component gallery)
- serde/serde_json (theme serialization)

---

## Phase 1: Project Structure Setup

### Task 1: Initialize Workspace

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`
- Create: `README.md`

**Step 1: Create workspace Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
    "crates/ui",
    "crates/gallery",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
authors = ["Your Name <email@example.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/gpui-flutter"

[workspace.dependencies]
# GPUI
gpui = "0.2"

# Icons
lucide-icons = "0.555"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Utilities
anyhow = "1.0"
thiserror = "1.0"
```

Write this to `Cargo.toml`.

**Step 2: Create .gitignore**

```gitignore
# Rust
/target/
**/*.rs.bk
Cargo.lock

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Build artifacts
*.pdb
*.dll
*.dylib
*.so
```

Write this to `.gitignore`.

**Step 3: Create README.md**

```markdown
# gpui-flutter

Flutter-style component library for GPUI with shadcn/ui design system.

## Features

- 🎨 shadcn/ui inspired design tokens
- 🔧 Flutter-style API (Container, Row, Column, etc.)
- ♿ Built-in accessibility (ARIA)
- 🎭 Lucide icons integration
- 🌗 Light/Dark theme support
- 📚 Storybook gallery

## Quick Start

\`\`\`bash
# Run the component gallery
cargo run -p gpui-flutter-gallery
\`\`\`

## Components

### Layout
- Container
- Row / Column
- Stack
- Spacer

### Buttons
- Button
- IconButton

### Inputs
- TextField
- Checkbox
- Switch

### Cards
- Card

*More components coming soon...*

## License

MIT OR Apache-2.0
```

Write this to `README.md`.

**Step 4: Initialize git repository**

Run:
```bash
git init
git add .
git commit -m "chore: initialize workspace"
```

Expected: Repository initialized with workspace structure.

---

### Task 2: Create UI Crate Structure

**Files:**
- Create: `crates/ui/Cargo.toml`
- Create: `crates/ui/src/lib.rs`
- Create: `crates/ui/src/prelude.rs`

**Step 1: Create ui crate Cargo.toml**

```toml
[package]
name = "gpui-flutter"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true

[dependencies]
gpui.workspace = true
lucide-icons.workspace = true
serde.workspace = true
serde_json.workspace = true
anyhow.workspace = true
thiserror.workspace = true

[features]
default = []
```

Write this to `crates/ui/Cargo.toml`.

**Step 2: Create lib.rs with module structure**

```rust
// Core modules
pub mod theme;
pub mod icons;
pub mod accessibility;
pub mod styled_ext;

// Component modules
pub mod components;

// Prelude for convenient imports
pub mod prelude;

use gpui::App;

/// Initialize the gpui-flutter library
pub fn init(cx: &mut gpui::AppContext) {
    // Register any global state or actions here
    theme::init(cx);
}
```

Write this to `crates/ui/src/lib.rs`.

**Step 3: Create prelude.rs**

```rust
//! Prelude module for convenient imports

// Re-export GPUI essentials
pub use gpui::{
    div, px, point, size, bounds, hsla, rgb, rgba,
    App, AppContext, WindowContext, Context,
    IntoElement, RenderOnce, Render, Styled,
    SharedString, AnyElement, ElementId,
    Div, Pixels, Hsla, BoxShadow,
    Window, WindowOptions, WindowBounds, Bounds,
    TitlebarOptions,
};

// Re-export our modules
pub use crate::theme::{Theme, ThemeColors, ThemeRegistry, ActiveTheme};
pub use crate::icons::{Icon, IconSize};
pub use crate::styled_ext::StyledExt;
pub use crate::accessibility::{Accessible, AccessibilityProps, AriaRole};

// Re-export components
pub use crate::components::*;
```

Write this to `crates/ui/src/prelude.rs`.

**Step 4: Verify crate builds**

Run:
```bash
cd crates/ui
cargo check
```

Expected: Compilation errors about missing modules (normal, we'll create them next).

**Step 5: Commit**

```bash
git add crates/ui/
git commit -m "feat: create ui crate structure"
```

---

## Phase 2: Theme System

### Task 3: Theme Data Structures

**Files:**
- Create: `crates/ui/src/theme/mod.rs`
- Create: `crates/ui/src/theme/colors.rs`
- Create: `crates/ui/src/theme/types.rs`

**Step 1: Create theme types**

```rust
// crates/ui/src/theme/types.rs

use gpui::{Hsla, Pixels, px};
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
    pub xs: f32,   // 4px
    pub sm: f32,   // 8px
    pub md: f32,   // 16px
    pub lg: f32,   // 24px
    pub xl: f32,   // 32px
    pub xxl: f32,  // 48px
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
```

Write this to `crates/ui/src/theme/types.rs`.

**Step 2: Create colors module with Hsla conversion**

```rust
// crates/ui/src/theme/colors.rs

use gpui::{Hsla, hsla};

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
```

Write this to `crates/ui/src/theme/colors.rs`.

**Step 3: Create theme mod.rs**

```rust
// crates/ui/src/theme/mod.rs

mod types;
mod colors;
mod registry;

pub use types::*;
pub use colors::ThemeColors;
pub use registry::{ThemeRegistry, ActiveTheme};

use gpui::{AppContext, Pixels, px};

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
pub fn init(_cx: &mut AppContext) {
    // Initialization logic if needed
}
```

Write this to `crates/ui/src/theme/mod.rs`.

**Step 4: Run tests**

Run:
```bash
cargo test -p gpui-flutter --lib theme::colors::tests
```

Expected: 1 test passes for HSL parsing.

**Step 5: Commit**

```bash
git add crates/ui/src/theme/
git commit -m "feat(theme): add theme data structures and color parsing"
```

---

### Task 4: Theme Registry

**Files:**
- Create: `crates/ui/src/theme/registry.rs`
- Create: `crates/ui/themes/default-light.json`
- Create: `crates/ui/themes/default-dark.json`

**Step 1: Create theme registry**

```rust
// crates/ui/src/theme/registry.rs

use super::{Theme, ThemeConfig, ThemeMode};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use gpui::{AppContext, WindowContext};

static THEME_REGISTRY: once_cell::sync::Lazy<Arc<RwLock<ThemeRegistryInner>>> =
    once_cell::sync::Lazy::new(|| {
        Arc::new(RwLock::new(ThemeRegistryInner::new()))
    });

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
        registry.themes
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

impl ActiveTheme for WindowContext<'_> {
    fn theme(&self) -> Theme {
        ThemeRegistry::get_active()
    }
}

impl ActiveTheme for AppContext {
    fn theme(&self) -> Theme {
        ThemeRegistry::get_active()
    }
}
```

Write this to `crates/ui/src/theme/registry.rs`.

**Step 2: Add once_cell dependency**

Add to `crates/ui/Cargo.toml` dependencies:
```toml
once_cell = "1.19"
```

**Step 3: Create default light theme JSON**

```json
{
  "name": "default-light",
  "mode": "light",
  "colors": {
    "background": "0 0% 100%",
    "foreground": "222 84% 5%",
    "primary": "222 47% 11%",
    "primary_foreground": "210 40% 98%",
    "secondary": "210 40% 96%",
    "secondary_foreground": "222 47% 11%",
    "muted": "210 40% 96%",
    "muted_foreground": "215 16% 47%",
    "accent": "210 40% 96%",
    "accent_foreground": "222 47% 11%",
    "destructive": "0 84% 60%",
    "destructive_foreground": "210 40% 98%",
    "border": "214 32% 91%",
    "input": "214 32% 91%",
    "ring": "222 84% 5%"
  },
  "radius": {
    "sm": 4.0,
    "md": 8.0,
    "lg": 12.0,
    "full": 9999.0
  },
  "spacing": {
    "xs": 4.0,
    "sm": 8.0,
    "md": 16.0,
    "lg": 24.0,
    "xl": 32.0,
    "xxl": 48.0
  },
  "typography": {
    "font_family": "system-ui, -apple-system, sans-serif",
    "font_mono": "ui-monospace, monospace",
    "size_xs": 12.0,
    "size_sm": 14.0,
    "size_base": 16.0,
    "size_lg": 18.0,
    "size_xl": 20.0,
    "size_2xl": 24.0,
    "weight_normal": 400,
    "weight_medium": 500,
    "weight_semibold": 600,
    "weight_bold": 700
  }
}
```

Write this to `crates/ui/themes/default-light.json`.

**Step 4: Create default dark theme JSON**

```json
{
  "name": "default-dark",
  "mode": "dark",
  "colors": {
    "background": "222 84% 5%",
    "foreground": "210 40% 98%",
    "primary": "210 40% 98%",
    "primary_foreground": "222 47% 11%",
    "secondary": "217 33% 17%",
    "secondary_foreground": "210 40% 98%",
    "muted": "217 33% 17%",
    "muted_foreground": "215 20% 65%",
    "accent": "217 33% 17%",
    "accent_foreground": "210 40% 98%",
    "destructive": "0 63% 31%",
    "destructive_foreground": "210 40% 98%",
    "border": "217 33% 17%",
    "input": "217 33% 17%",
    "ring": "212 93% 87%"
  },
  "radius": {
    "sm": 4.0,
    "md": 8.0,
    "lg": 12.0,
    "full": 9999.0
  },
  "spacing": {
    "xs": 4.0,
    "sm": 8.0,
    "md": 16.0,
    "lg": 24.0,
    "xl": 32.0,
    "xxl": 48.0
  },
  "typography": {
    "font_family": "system-ui, -apple-system, sans-serif",
    "font_mono": "ui-monospace, monospace",
    "size_xs": 12.0,
    "size_sm": 14.0,
    "size_base": 16.0,
    "size_lg": 18.0,
    "size_xl": 20.0,
    "size_2xl": 24.0,
    "weight_normal": 400,
    "weight_medium": 500,
    "weight_semibold": 600,
    "weight_bold": 700
  }
}
```

Write this to `crates/ui/themes/default-dark.json`.

**Step 5: Load themes on init**

Update `crates/ui/src/theme/mod.rs` init function:

```rust
pub fn init(_cx: &mut AppContext) {
    // Load default themes
    let light_theme = include_str!("../themes/default-light.json");
    let dark_theme = include_str!("../themes/default-dark.json");
    
    ThemeRegistry::register_from_json(light_theme)
        .expect("Failed to load default light theme");
    ThemeRegistry::register_from_json(dark_theme)
        .expect("Failed to load default dark theme");
}
```

**Step 6: Verify theme loading**

Run:
```bash
cargo check -p gpui-flutter
```

Expected: Should compile successfully.

**Step 7: Commit**

```bash
git add crates/ui/src/theme/registry.rs crates/ui/themes/ crates/ui/Cargo.toml
git commit -m "feat(theme): add theme registry and default themes"
```

---

## Phase 3: Core Utilities

### Task 5: StyledExt Trait

**Files:**
- Create: `crates/ui/src/styled_ext.rs`

**Step 1: Create StyledExt trait**

```rust
// crates/ui/src/styled_ext.rs

use gpui::{Styled, Div, div, Pixels, px};

/// Extension trait for convenient styling methods
pub trait StyledExt: Styled + Sized {
    // Layout helpers
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }
    
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }
    
    // Gap utilities
    fn gap_1(self) -> Self {
        self.gap(px(4.))
    }
    
    fn gap_2(self) -> Self {
        self.gap(px(8.))
    }
    
    fn gap_3(self) -> Self {
        self.gap(px(12.))
    }
    
    fn gap_4(self) -> Self {
        self.gap(px(16.))
    }
    
    fn gap_6(self) -> Self {
        self.gap(px(24.))
    }
    
    fn gap_8(self) -> Self {
        self.gap(px(32.))
    }
    
    // Padding utilities
    fn p_1(self) -> Self {
        self.p(px(4.))
    }
    
    fn p_2(self) -> Self {
        self.p(px(8.))
    }
    
    fn p_3(self) -> Self {
        self.p(px(12.))
    }
    
    fn p_4(self) -> Self {
        self.p(px(16.))
    }
    
    fn p_6(self) -> Self {
        self.p(px(24.))
    }
    
    fn p_8(self) -> Self {
        self.p(px(32.))
    }
    
    fn px_1(self) -> Self {
        self.px(px(4.))
    }
    
    fn px_2(self) -> Self {
        self.px(px(8.))
    }
    
    fn px_3(self) -> Self {
        self.px(px(12.))
    }
    
    fn px_4(self) -> Self {
        self.px(px(16.))
    }
    
    fn py_1(self) -> Self {
        self.py(px(4.))
    }
    
    fn py_2(self) -> Self {
        self.py(px(8.))
    }
    
    fn py_3(self) -> Self {
        self.py(px(12.))
    }
    
    fn py_4(self) -> Self {
        self.py(px(16.))
    }
    
    // Border radius utilities
    fn rounded_sm(self) -> Self {
        self.rounded(px(4.))
    }
    
    fn rounded_md(self) -> Self {
        self.rounded(px(8.))
    }
    
    fn rounded_lg(self) -> Self {
        self.rounded(px(12.))
    }
    
    fn rounded_full(self) -> Self {
        self.rounded(px(9999.))
    }
}

// Implement for all Styled types
impl<T: Styled> StyledExt for T {}
```

Write this to `crates/ui/src/styled_ext.rs`.

**Step 2: Verify compilation**

Run:
```bash
cargo check -p gpui-flutter
```

Expected: Compiles successfully.

**Step 3: Commit**

```bash
git add crates/ui/src/styled_ext.rs
git commit -m "feat: add StyledExt trait for convenient styling"
```

---

### Task 6: Accessibility Module

**Files:**
- Create: `crates/ui/src/accessibility/mod.rs`

**Step 1: Create accessibility types**

```rust
// crates/ui/src/accessibility/mod.rs

use gpui::{Styled, SharedString, ElementId};

#[derive(Clone, Default)]
pub struct AccessibilityProps {
    pub role: Option<AriaRole>,
    pub label: Option<SharedString>,
    pub description: Option<SharedString>,
    pub hidden: bool,
    pub expanded: Option<bool>,
    pub selected: Option<bool>,
    pub disabled: bool,
    pub required: bool,
    pub invalid: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaRole {
    Button,
    Textbox,
    Checkbox,
    Radio,
    Dialog,
    Alert,
    Navigation,
    Main,
}

/// Trait for applying accessibility to elements
pub trait Accessible: Styled + Sized {
    fn accessibility(self, _props: AccessibilityProps) -> Self {
        // GPUI's accessibility will be implemented here
        // For now, this is a placeholder
        self
    }
    
    fn aria_label(self, label: impl Into<SharedString>) -> Self {
        let mut props = AccessibilityProps::default();
        props.label = Some(label.into());
        self.accessibility(props)
    }
    
    fn aria_hidden(self, hidden: bool) -> Self {
        let mut props = AccessibilityProps::default();
        props.hidden = hidden;
        self.accessibility(props)
    }
    
    fn aria_disabled(self, disabled: bool) -> Self {
        let mut props = AccessibilityProps::default();
        props.disabled = disabled;
        self.accessibility(props)
    }
}

// Implement for Div
impl Accessible for gpui::Div {}
```

Write this to `crates/ui/src/accessibility/mod.rs`.

**Step 2: Verify compilation**

Run:
```bash
cargo check -p gpui-flutter
```

Expected: Compiles successfully.

**Step 3: Commit**

```bash
git add crates/ui/src/accessibility/
git commit -m "feat: add accessibility module with ARIA support"
```

---

## Phase 4: Icons System

### Task 7: Icon Component

**Files:**
- Create: `crates/ui/src/icons/mod.rs`

**Step 1: Create Icon component**

```rust
// crates/ui/src/icons/mod.rs

use gpui::*;
pub use lucide_icons::Icon as LucideIcon;

#[derive(IntoElement, Clone)]
pub struct Icon {
    icon: LucideIcon,
    size: IconSize,
    color: Option<Hsla>,
    
    // Accessibility
    label: Option<SharedString>,
    decorative: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum IconSize {
    Xs,  // 12px
    Sm,  // 16px
    Md,  // 20px
    Lg,  // 24px
    Xl,  // 32px
}

impl IconSize {
    pub fn to_pixels(&self) -> Pixels {
        match self {
            IconSize::Xs => px(12.),
            IconSize::Sm => px(16.),
            IconSize::Md => px(20.),
            IconSize::Lg => px(24.),
            IconSize::Xl => px(32.),
        }
    }
}

impl Icon {
    pub fn new(icon: LucideIcon) -> Self {
        Self {
            icon,
            size: IconSize::Lg,
            color: None,
            label: None,
            decorative: false,
        }
    }
    
    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }
    
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
    
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self.decorative = false;
        self
    }
    
    pub fn decorative(mut self) -> Self {
        self.decorative = true;
        self.label = None;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.theme();
        let color = self.color.unwrap_or(theme.colors.foreground);
        let size = self.size.to_pixels();
        
        // For now, render as a simple colored box
        // TODO: Implement actual SVG rendering when GPUI supports it
        div()
            .size(size)
            .bg(color)
            .rounded(px(2.))
            .flex_none()
    }
}
```

Write this to `crates/ui/src/icons/mod.rs`.

**Step 2: Verify compilation**

Run:
```bash
cargo check -p gpui-flutter
```

Expected: Compiles successfully.

**Step 3: Commit**

```bash
git add crates/ui/src/icons/
git commit -m "feat: add Icon component with Lucide support"
```

---

## Phase 5: Components Module Setup

### Task 8: Components Module Structure

**Files:**
- Create: `crates/ui/src/components/mod.rs`
- Create: `crates/ui/src/components/container.rs`
- Create: `crates/ui/src/components/row.rs`
- Create: `crates/ui/src/components/column.rs`
- Create: `crates/ui/src/components/text.rs`
- Create: `crates/ui/src/components/button.rs`

**Step 1: Create components mod.rs**

```rust
// crates/ui/src/components/mod.rs

mod container;
mod row;
mod column;
mod text;
mod button;

pub use container::Container;
pub use row::Row;
pub use column::Column;
pub use text::Text;
pub use button::{Button, ButtonVariant, ButtonSize};
```

Write this to `crates/ui/src/components/mod.rs`.

**Step 2: Create placeholder files**

```rust
// crates/ui/src/components/container.rs
pub struct Container;
```

```rust
// crates/ui/src/components/row.rs
pub struct Row;
```

```rust
// crates/ui/src/components/column.rs
pub struct Column;
```

```rust
// crates/ui/src/components/text.rs
pub struct Text;
```

```rust
// crates/ui/src/components/button.rs
pub struct Button;
pub enum ButtonVariant {}
pub enum ButtonSize {}
```

**Step 3: Verify compilation**

Run:
```bash
cargo check -p gpui-flutter
```

Expected: Compiles successfully with warnings about unused items.

**Step 4: Commit**

```bash
git add crates/ui/src/components/
git commit -m "feat: create components module structure"
```

---

## Next Steps

Plan continues with implementation of each component in Phase 6+. The structure is now ready for implementing:

- Container component (Flutter-style)
- Row/Column layout components
- Text component
- Button component with variants

**Current Status:** ✅ Project structure complete, theme system ready, utilities in place.

**Ready to implement:** Individual components with Flutter-style API.

---

## Plan complete!

This plan sets up the foundation. Next plan will cover implementing the actual components (Container, Row, Column, Text, Button) with full Flutter-style API, tests, and stories for the gallery.
