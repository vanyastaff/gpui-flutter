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
