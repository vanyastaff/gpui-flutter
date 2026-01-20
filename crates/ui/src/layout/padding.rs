// crates/ui/src/layout/padding.rs

use gpui::*;

/// Flutter-style Padding widget
#[derive(IntoElement)]
pub struct Padding {
    child: AnyElement,
    padding: EdgeInsets,
}

#[derive(Clone, Copy, Default)]
struct EdgeInsets {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

impl Padding {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
            padding: EdgeInsets::default(),
        }
    }

    /// Set padding on all sides
    pub fn all(mut self, value: f32) -> Self {
        self.padding = EdgeInsets {
            top: value,
            right: value,
            bottom: value,
            left: value,
        };
        self
    }

    /// Set horizontal padding (left and right)
    pub fn horizontal(mut self, value: f32) -> Self {
        self.padding.left = value;
        self.padding.right = value;
        self
    }

    /// Set vertical padding (top and bottom)
    pub fn vertical(mut self, value: f32) -> Self {
        self.padding.top = value;
        self.padding.bottom = value;
        self
    }

    /// Set symmetric padding (horizontal and vertical)
    pub fn symmetric(self, horizontal: f32, vertical: f32) -> Self {
        self.horizontal(horizontal).vertical(vertical)
    }

    /// Set padding for left edge
    pub fn left(mut self, value: f32) -> Self {
        self.padding.left = value;
        self
    }

    /// Set padding for right edge
    pub fn right(mut self, value: f32) -> Self {
        self.padding.right = value;
        self
    }

    /// Set padding for top edge
    pub fn top(mut self, value: f32) -> Self {
        self.padding.top = value;
        self
    }

    /// Set padding for bottom edge
    pub fn bottom(mut self, value: f32) -> Self {
        self.padding.bottom = value;
        self
    }
}

impl RenderOnce for Padding {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .pt(px(self.padding.top))
            .pr(px(self.padding.right))
            .pb(px(self.padding.bottom))
            .pl(px(self.padding.left))
            .child(self.child)
    }
}
