// crates/ui/src/color.rs

use gpui::{hsla, rgba, Hsla, Rgba};

/// Flutter-style Color type that can be created from multiple formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    inner: Hsla,
}

impl Color {
    // ============================================
    // Constructors (Flutter-style)
    // ============================================

    /// Create from RGBA (0-255 for RGB, 0.0-1.0 for alpha)
    ///
    /// # Example
    /// ```
    /// let red = Color::from_rgba(255, 0, 0, 1.0);
    /// ```
    pub fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        let rgba_val = rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a);
        Self {
            inner: Hsla::from(rgba_val),
        }
    }

    /// Create from RGB (0-255), alpha = 1.0
    ///
    /// # Example
    /// ```
    /// let red = Color::rgb(255, 0, 0);
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 1.0)
    }

    /// Create from ARGB hex (like Flutter's Color(0xFFFF0000))
    ///
    /// # Example
    /// ```
    /// let red = Color::from_argb(0xFFFF0000);
    /// ```
    pub fn from_argb(argb: u32) -> Self {
        let a = ((argb >> 24) & 0xFF) as u8;
        let r = ((argb >> 16) & 0xFF) as u8;
        let g = ((argb >> 8) & 0xFF) as u8;
        let b = (argb & 0xFF) as u8;

        Self::from_rgba(r, g, b, a as f32 / 255.0)
    }

    /// Create from RGB hex (assumes alpha = 1.0)
    ///
    /// # Example
    /// ```
    /// let red = Color::from_hex(0xFF0000);
    /// ```
    pub fn from_hex(hex: u32) -> Self {
        Self::from_argb(0xFF000000 | hex)
    }

    /// Create from HSL (hue: 0-360, saturation: 0-100, lightness: 0-100)
    ///
    /// # Example
    /// ```
    /// let blue = Color::from_hsl(240.0, 100.0, 50.0);
    /// ```
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        Self {
            inner: hsla(h / 360.0, s / 100.0, l / 100.0, 1.0),
        }
    }

    /// Create from HSLA
    pub fn from_hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self {
            inner: hsla(h / 360.0, s / 100.0, l / 100.0, a),
        }
    }

    /// Parse HSL string "hue saturation% lightness%" to Color
    ///
    /// # Example
    /// ```
    /// let color = Color::parse_hsl("222 47% 11%");
    /// ```
    pub fn parse_hsl(hsl_str: &str) -> Self {
        let parts: Vec<&str> = hsl_str.split_whitespace().collect();

        if parts.len() != 3 {
            eprintln!("Invalid HSL format: {}, using fallback black", hsl_str);
            return Self::black();
        }

        let hue: f32 = parts[0].parse().unwrap_or(0.0);
        let saturation: f32 = parts[1].trim_end_matches('%').parse().unwrap_or(0.0);
        let lightness: f32 = parts[2].trim_end_matches('%').parse().unwrap_or(0.0);

        Self::from_hsl(hue, saturation, lightness)
    }

    // ============================================
    // Named Colors (Material Design palette)
    // ============================================

    pub fn transparent() -> Self {
        Self {
            inner: hsla(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn black() -> Self {
        Self::rgb(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }

    // Material Red
    pub fn red() -> Self {
        Self::from_hex(0xF44336)
    }

    pub fn red_50() -> Self {
        Self::from_hex(0xFFEBEE)
    }

    pub fn red_100() -> Self {
        Self::from_hex(0xFFCDD2)
    }

    pub fn red_200() -> Self {
        Self::from_hex(0xEF9A9A)
    }

    pub fn red_300() -> Self {
        Self::from_hex(0xE57373)
    }

    pub fn red_400() -> Self {
        Self::from_hex(0xEF5350)
    }

    pub fn red_500() -> Self {
        Self::from_hex(0xF44336)
    }

    pub fn red_600() -> Self {
        Self::from_hex(0xE53935)
    }

    pub fn red_700() -> Self {
        Self::from_hex(0xD32F2F)
    }

    pub fn red_800() -> Self {
        Self::from_hex(0xC62828)
    }

    pub fn red_900() -> Self {
        Self::from_hex(0xB71C1C)
    }

    // Material Blue
    pub fn blue() -> Self {
        Self::from_hex(0x2196F3)
    }

    pub fn blue_50() -> Self {
        Self::from_hex(0xE3F2FD)
    }

    pub fn blue_100() -> Self {
        Self::from_hex(0xBBDEFB)
    }

    pub fn blue_200() -> Self {
        Self::from_hex(0x90CAF9)
    }

    pub fn blue_300() -> Self {
        Self::from_hex(0x64B5F6)
    }

    pub fn blue_400() -> Self {
        Self::from_hex(0x42A5F5)
    }

    pub fn blue_500() -> Self {
        Self::from_hex(0x2196F3)
    }

    pub fn blue_600() -> Self {
        Self::from_hex(0x1E88E5)
    }

    pub fn blue_700() -> Self {
        Self::from_hex(0x1976D2)
    }

    pub fn blue_800() -> Self {
        Self::from_hex(0x1565C0)
    }

    pub fn blue_900() -> Self {
        Self::from_hex(0x0D47A1)
    }

    // Material Green
    pub fn green() -> Self {
        Self::from_hex(0x4CAF50)
    }

    pub fn green_50() -> Self {
        Self::from_hex(0xE8F5E9)
    }

    pub fn green_100() -> Self {
        Self::from_hex(0xC8E6C9)
    }

    pub fn green_500() -> Self {
        Self::from_hex(0x4CAF50)
    }

    pub fn green_700() -> Self {
        Self::from_hex(0x388E3C)
    }

    pub fn green_900() -> Self {
        Self::from_hex(0x1B5E20)
    }

    // Material Yellow/Amber
    pub fn yellow() -> Self {
        Self::from_hex(0xFFEB3B)
    }

    pub fn amber() -> Self {
        Self::from_hex(0xFFC107)
    }

    pub fn amber_500() -> Self {
        Self::from_hex(0xFFC107)
    }

    // Material Orange
    pub fn orange() -> Self {
        Self::from_hex(0xFF9800)
    }

    pub fn orange_500() -> Self {
        Self::from_hex(0xFF9800)
    }

    pub fn deep_orange() -> Self {
        Self::from_hex(0xFF5722)
    }

    // Material Purple
    pub fn purple() -> Self {
        Self::from_hex(0x9C27B0)
    }

    pub fn purple_500() -> Self {
        Self::from_hex(0x9C27B0)
    }

    pub fn deep_purple() -> Self {
        Self::from_hex(0x673AB7)
    }

    // Material Pink
    pub fn pink() -> Self {
        Self::from_hex(0xE91E63)
    }

    pub fn pink_500() -> Self {
        Self::from_hex(0xE91E63)
    }

    // Material Teal
    pub fn teal() -> Self {
        Self::from_hex(0x009688)
    }

    pub fn teal_500() -> Self {
        Self::from_hex(0x009688)
    }

    // Material Cyan
    pub fn cyan() -> Self {
        Self::from_hex(0x00BCD4)
    }

    pub fn cyan_500() -> Self {
        Self::from_hex(0x00BCD4)
    }

    // Grey scale
    pub fn grey() -> Self {
        Self::rgb(158, 158, 158)
    }

    pub fn grey_50() -> Self {
        Self::from_hex(0xFAFAFA)
    }

    pub fn grey_100() -> Self {
        Self::from_hex(0xF5F5F5)
    }

    pub fn grey_200() -> Self {
        Self::from_hex(0xEEEEEE)
    }

    pub fn grey_300() -> Self {
        Self::from_hex(0xE0E0E0)
    }

    pub fn grey_400() -> Self {
        Self::from_hex(0xBDBDBD)
    }

    pub fn grey_500() -> Self {
        Self::from_hex(0x9E9E9E)
    }

    pub fn grey_600() -> Self {
        Self::from_hex(0x757575)
    }

    pub fn grey_700() -> Self {
        Self::from_hex(0x616161)
    }

    pub fn grey_800() -> Self {
        Self::from_hex(0x424242)
    }

    pub fn grey_900() -> Self {
        Self::from_hex(0x212121)
    }

    // ============================================
    // Modifiers (return new Color)
    // ============================================

    /// Create a new color with modified opacity
    ///
    /// # Example
    /// ```
    /// let semi_transparent_red = Color::red().with_opacity(0.5);
    /// ```
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.inner.a = opacity.clamp(0.0, 1.0);
        self
    }

    /// Create a new color with modified alpha (alias for with_opacity)
    pub fn with_alpha(self, alpha: f32) -> Self {
        self.with_opacity(alpha)
    }

    /// Darken the color by a percentage (0.0-1.0)
    pub fn darken(mut self, amount: f32) -> Self {
        self.inner.l = (self.inner.l - amount).max(0.0);
        self
    }

    /// Lighten the color by a percentage (0.0-1.0)
    pub fn lighten(mut self, amount: f32) -> Self {
        self.inner.l = (self.inner.l + amount).min(1.0);
        self
    }

    // ============================================
    // Conversions to GPUI types
    // ============================================

    /// Convert to GPUI Hsla
    pub fn to_hsla(&self) -> Hsla {
        self.inner
    }

    /// Convert to GPUI Rgba
    pub fn to_rgba(&self) -> Rgba {
        Rgba::from(self.inner)
    }
}

// ============================================
// Trait implementations for easy conversion
// ============================================

impl From<Color> for Hsla {
    fn from(color: Color) -> Self {
        color.inner
    }
}

impl From<Color> for Rgba {
    fn from(color: Color) -> Self {
        Rgba::from(color.inner)
    }
}

impl From<Hsla> for Color {
    fn from(hsla: Hsla) -> Self {
        Self { inner: hsla }
    }
}

impl From<Rgba> for Color {
    fn from(rgba: Rgba) -> Self {
        Self {
            inner: Hsla::from(rgba),
        }
    }
}

// Allow using Color directly where Hsla is expected
impl From<&Color> for Hsla {
    fn from(color: &Color) -> Self {
        color.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_constructor() {
        let red = Color::rgb(255, 0, 0);
        let rgba = red.to_rgba();
        assert!((rgba.r - 1.0).abs() < 0.01);
        assert!((rgba.g - 0.0).abs() < 0.01);
        assert!((rgba.b - 0.0).abs() < 0.01);
        assert!((rgba.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_hex_constructor() {
        let red = Color::from_hex(0xFF0000);
        let rgba = red.to_rgba();
        assert!((rgba.r - 1.0).abs() < 0.01);
        assert!((rgba.g - 0.0).abs() < 0.01);
        assert!((rgba.b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_argb_constructor() {
        let semi_red = Color::from_argb(0x80FF0000);
        let rgba = semi_red.to_rgba();
        assert!((rgba.a - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_with_opacity() {
        let red = Color::red();
        let semi_red = red.with_opacity(0.5);
        assert!((semi_red.to_rgba().a - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_named_colors() {
        let red = Color::red();
        let blue = Color::blue();
        let green = Color::green();

        // Just verify they're different
        assert_ne!(red.to_hsla(), blue.to_hsla());
        assert_ne!(blue.to_hsla(), green.to_hsla());
    }

    #[test]
    fn test_parse_hsl() {
        let color = Color::parse_hsl("222 47% 11%");
        let hsla = color.to_hsla();
        // hue: 222/360 = 0.617
        // sat: 47% = 0.47
        // light: 11% = 0.11
        assert!((hsla.h - 0.617).abs() < 0.01);
        assert!((hsla.s - 0.47).abs() < 0.01);
        assert!((hsla.l - 0.11).abs() < 0.01);
        assert_eq!(hsla.a, 1.0);
    }
}
