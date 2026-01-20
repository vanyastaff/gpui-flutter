// crates/ui/src/color.rs

use gpui::{hsla, Hsla, Rgba};
use std::str::FromStr;

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
        let rgba_val = Rgba {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        };
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

    // ============================================
    // Internal parsing helpers
    // ============================================

    /// Parse HSL string "hue saturation% lightness%" to Color
    /// Used internally for theme parsing
    pub(crate) fn parse_hsl_compact(hsl_str: &str) -> Self {
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

    /// Parse hex string (without #)
    fn parse_hex_string(hex: &str) -> Result<Self, ColorParseError> {
        let hex = hex.trim();

        // #RGB -> #RRGGBB
        let expanded = if hex.len() == 3 {
            hex.chars()
                .map(|c| format!("{}{}", c, c))
                .collect::<String>()
        } else {
            hex.to_string()
        };

        if expanded.len() != 6 && expanded.len() != 8 {
            return Err(ColorParseError::InvalidFormat);
        }

        let value = u32::from_str_radix(&expanded, 16).map_err(|_| ColorParseError::InvalidHex)?;

        Ok(if expanded.len() == 8 {
            Self::from_argb(value)
        } else {
            Self::from_hex(value)
        })
    }

    /// Parse hsl() function format
    fn parse_hsl_function(s: &str) -> Result<Self, ColorParseError> {
        let s = s
            .trim()
            .trim_start_matches("hsl(")
            .trim_end_matches(')')
            .replace(',', " ");

        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(ColorParseError::InvalidFormat);
        }

        let hue: f32 = parts[0]
            .parse()
            .map_err(|_| ColorParseError::InvalidNumber)?;
        let saturation: f32 = parts[1]
            .trim_end_matches('%')
            .parse()
            .map_err(|_| ColorParseError::InvalidNumber)?;
        let lightness: f32 = parts[2]
            .trim_end_matches('%')
            .parse()
            .map_err(|_| ColorParseError::InvalidNumber)?;

        Ok(Self::from_hsl(hue, saturation, lightness))
    }

    /// Parse rgb() function format
    fn parse_rgb_function(s: &str) -> Result<Self, ColorParseError> {
        let inner = s.trim().trim_start_matches("rgb(").trim_end_matches(')');

        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() != 3 {
            return Err(ColorParseError::InvalidFormat);
        }

        let r: u8 = parts[0]
            .parse()
            .map_err(|_| ColorParseError::InvalidNumber)?;
        let g: u8 = parts[1]
            .parse()
            .map_err(|_| ColorParseError::InvalidNumber)?;
        let b: u8 = parts[2]
            .parse()
            .map_err(|_| ColorParseError::InvalidNumber)?;

        Ok(Self::rgb(r, g, b))
    }

    /// Parse named color
    fn parse_named_color(name: &str) -> Result<Self, ColorParseError> {
        match name.to_lowercase().as_str() {
            "transparent" => Ok(Self::transparent()),
            "black" => Ok(Self::black()),
            "white" => Ok(Self::white()),
            "red" => Ok(Self::red()),
            "blue" => Ok(Self::blue()),
            "green" => Ok(Self::green()),
            "yellow" => Ok(Self::yellow()),
            "orange" => Ok(Self::orange()),
            "purple" => Ok(Self::purple()),
            "pink" => Ok(Self::pink()),
            "grey" | "gray" => Ok(Self::grey()),
            "cyan" => Ok(Self::cyan()),
            "teal" => Ok(Self::teal()),
            "amber" => Ok(Self::amber()),
            _ => Err(ColorParseError::UnknownColor),
        }
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

    /// Saturate the color by a percentage (0.0-1.0)
    pub fn saturate(mut self, amount: f32) -> Self {
        self.inner.s = (self.inner.s + amount).min(1.0);
        self
    }

    /// Desaturate the color by a percentage (0.0-1.0)
    pub fn desaturate(mut self, amount: f32) -> Self {
        self.inner.s = (self.inner.s - amount).max(0.0);
        self
    }

    /// Adjust hue by degrees (-360 to 360)
    pub fn rotate_hue(mut self, degrees: f32) -> Self {
        self.inner.h = (self.inner.h + degrees / 360.0) % 1.0;
        if self.inner.h < 0.0 {
            self.inner.h += 1.0;
        }
        self
    }

    /// Mix this color with another color
    pub fn mix(self, other: Color, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let self_rgba = self.to_rgba();
        let other_rgba = other.to_rgba();

        Self::from_rgba(
            ((self_rgba.r * (1.0 - ratio) + other_rgba.r * ratio) * 255.0) as u8,
            ((self_rgba.g * (1.0 - ratio) + other_rgba.g * ratio) * 255.0) as u8,
            ((self_rgba.b * (1.0 - ratio) + other_rgba.b * ratio) * 255.0) as u8,
            self_rgba.a * (1.0 - ratio) + other_rgba.a * ratio,
        )
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

    /// Convert to hex string (without #)
    pub fn to_hex_string(&self) -> String {
        let rgba = self.to_rgba();
        format!(
            "{:02x}{:02x}{:02x}",
            (rgba.r * 255.0) as u8,
            (rgba.g * 255.0) as u8,
            (rgba.b * 255.0) as u8
        )
    }

    /// Convert to CSS rgb() string
    pub fn to_css_rgb(&self) -> String {
        let rgba = self.to_rgba();
        format!(
            "rgb({}, {}, {})",
            (rgba.r * 255.0) as u8,
            (rgba.g * 255.0) as u8,
            (rgba.b * 255.0) as u8
        )
    }

    /// Convert to CSS hsl() string
    pub fn to_css_hsl(&self) -> String {
        format!(
            "hsl({:.0}, {:.0}%, {:.0}%)",
            self.inner.h * 360.0,
            self.inner.s * 100.0,
            self.inner.l * 100.0
        )
    }
}

// ============================================
// Error types
// ============================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorParseError {
    InvalidFormat,
    InvalidHex,
    InvalidNumber,
    UnknownColor,
}

impl std::fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorParseError::InvalidFormat => write!(f, "Invalid color format"),
            ColorParseError::InvalidHex => write!(f, "Invalid hex color"),
            ColorParseError::InvalidNumber => write!(f, "Invalid number in color"),
            ColorParseError::UnknownColor => write!(f, "Unknown color name"),
        }
    }
}

impl std::error::Error for ColorParseError {}

// ============================================
// Standard Rust trait implementations
// ============================================

/// FromStr is the idiomatic way to parse strings in Rust
///
/// # Example
/// ```
/// use std::str::FromStr;
/// let color = Color::from_str("#ff0000")?;
/// let color: Color = "blue".parse()?;  // even more idiomatic!
/// ```
impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Hex color: #RRGGBB or #RGB
        if s.starts_with('#') {
            return Self::parse_hex_string(&s[1..]);
        }

        // HSL format: hsl(h, s%, l%) or h s% l%
        if s.starts_with("hsl(") || s.contains('%') {
            return Self::parse_hsl_function(s);
        }

        // RGB format: rgb(r, g, b)
        if s.starts_with("rgb(") {
            return Self::parse_rgb_function(s);
        }

        // Named color
        Self::parse_named_color(s)
    }
}

// Conversions from GPUI types
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

// Display trait for debugging
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.to_hex_string())
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
    fn test_parse_hsl_compact() {
        let color = Color::parse_hsl_compact("222 47% 11%");
        let hsla = color.to_hsla();
        // hue: 222/360 = 0.617
        // sat: 47% = 0.47
        // light: 11% = 0.11
        assert!((hsla.h - 0.617).abs() < 0.01);
        assert!((hsla.s - 0.47).abs() < 0.01);
        assert!((hsla.l - 0.11).abs() < 0.01);
        assert_eq!(hsla.a, 1.0);
    }

    #[test]
    fn test_from_str_hex() {
        let red = Color::from_str("#ff0000").unwrap();
        let rgba = red.to_rgba();
        assert!((rgba.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_from_str_short_hex() {
        let red = Color::from_str("#f00").unwrap();
        let rgba = red.to_rgba();
        assert!((rgba.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_from_str_rgb_function() {
        let red = Color::from_str("rgb(255, 0, 0)").unwrap();
        let rgba = red.to_rgba();
        assert!((rgba.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_from_str_named_color() {
        let red = Color::from_str("red").unwrap();
        assert_eq!(red, Color::red());
    }

    #[test]
    fn test_parse_idiomatic() {
        // The most idiomatic Rust way!
        let red: Color = "#ff0000".parse().unwrap();
        let rgba = red.to_rgba();
        assert!((rgba.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_mix_colors() {
        let red = Color::red();
        let blue = Color::blue();
        let purple = red.mix(blue, 0.5);

        // Mixed color should be different from both
        assert_ne!(purple, red);
        assert_ne!(purple, blue);
    }

    #[test]
    fn test_color_modifiers() {
        let color = Color::blue();
        let darker = color.darken(0.2);
        let lighter = color.lighten(0.2);
        let saturated = color.saturate(0.1);

        assert_ne!(color, darker);
        assert_ne!(color, lighter);
        assert_ne!(color, saturated);
    }
}
