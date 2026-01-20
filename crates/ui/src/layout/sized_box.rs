// crates/ui/src/layout/sized_box.rs

use gpui::*;

/// Flutter-style SizedBox widget
#[derive(IntoElement)]
pub struct SizedBox {
    width: Option<f32>,
    height: Option<f32>,
    child: Option<AnyElement>,
}

impl SizedBox {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            child: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(self, width: f32, height: f32) -> Self {
        self.width(width).height(height)
    }

    pub fn square(self, size: f32) -> Self {
        self.width(size).height(size)
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }

    /// Shrink to fit child
    pub fn shrink() -> Self {
        Self::new()
    }

    /// Expand to fill parent width
    pub fn expand_width() -> impl IntoElement {
        div().w_full()
    }

    /// Expand to fill parent height
    pub fn expand_height() -> impl IntoElement {
        div().h_full()
    }

    /// Expand to fill parent in both dimensions
    pub fn expand() -> impl IntoElement {
        div().w_full().h_full()
    }
}

impl Default for SizedBox {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SizedBox {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut element = div();

        if let Some(w) = self.width {
            element = element.w(px(w));
        }

        if let Some(h) = self.height {
            element = element.h(px(h));
        }

        if let Some(child) = self.child {
            element = element.child(child);
        }

        element
    }
}
