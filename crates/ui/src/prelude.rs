//! Prelude module for convenient imports

// Re-export GPUI essentials
pub use gpui::{
    bounds, div, hsla, point, px, rgb, rgba, size, AnyElement, App, AppContext, Bounds, BoxShadow,
    Context, Div, ElementId, Hsla, IntoElement, Pixels, Render, RenderOnce, SharedString, Styled,
    TitlebarOptions, Window, WindowBounds, WindowOptions,
};

// Re-export our modules
pub use crate::accessibility::{AccessibilityProps, Accessible, AriaRole};
pub use crate::color::Color;
pub use crate::icons::{Icon, IconSize};
pub use crate::styled_ext::StyledExt;
pub use crate::theme::{ActiveTheme, Theme, ThemeColors, ThemeRegistry};

// Re-export components
pub use crate::components::*;
