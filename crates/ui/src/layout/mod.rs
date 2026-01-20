// crates/ui/src/layout/mod.rs

mod align;
mod center;
mod flexible;
mod padding;
mod sized_box;
mod spacer;

pub use align::Align;
pub use center::Center;
pub use flexible::{Expanded, Flexible};
pub use padding::Padding;
pub use sized_box::SizedBox;
pub use spacer::Spacer;

/// High-level alignment enum for positioning elements (Flutter-style)
///
/// This is a semantic wrapper over GPUI's flexbox alignment primitives,
/// combining justify_content and align_items for common positioning patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// Determines how a flex child should fit within its allocated space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexFit {
    /// Child can be smaller than flex space (uses flex_grow without flex_basis: 0)
    Loose,
    /// Child must fill all flex space (uses flex_grow with flex_basis: 0)
    Tight,
}
