// crates/ui/src/layout/padding.rs

use gpui::*;

/// Flutter-style Padding widget
#[derive(IntoElement)]
pub struct Padding {
    child: AnyElement,
    padding: Edges<DefiniteLength>,
}

impl Padding {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
            padding: Edges::default(),
        }
    }

    /// Set padding on all sides
    pub fn all(mut self, value: f32) -> Self {
        self.padding = Edges {
            top: px(value).into(),
            right: px(value).into(),
            bottom: px(value).into(),
            left: px(value).into(),
        };
        self
    }

    /// Set horizontal padding (left and right)
    pub fn horizontal(mut self, value: f32) -> Self {
        self.padding.left = px(value).into();
        self.padding.right = px(value).into();
        self
    }

    /// Set vertical padding (top and bottom)
    pub fn vertical(mut self, value: f32) -> Self {
        self.padding.top = px(value).into();
        self.padding.bottom = px(value).into();
        self
    }

    /// Set symmetric padding (horizontal and vertical)
    pub fn symmetric(self, horizontal: f32, vertical: f32) -> Self {
        self.horizontal(horizontal).vertical(vertical)
    }

    /// Set padding for left edge
    pub fn left(mut self, value: f32) -> Self {
        self.padding.left = px(value).into();
        self
    }

    /// Set padding for right edge
    pub fn right(mut self, value: f32) -> Self {
        self.padding.right = px(value).into();
        self
    }

    /// Set padding for top edge
    pub fn top(mut self, value: f32) -> Self {
        self.padding.top = px(value).into();
        self
    }

    /// Set padding for bottom edge
    pub fn bottom(mut self, value: f32) -> Self {
        self.padding.bottom = px(value).into();
        self
    }
}

impl RenderOnce for Padding {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .pt(self.padding.top)
            .pr(self.padding.right)
            .pb(self.padding.bottom)
            .pl(self.padding.left)
            .child(self.child)
    }
}
