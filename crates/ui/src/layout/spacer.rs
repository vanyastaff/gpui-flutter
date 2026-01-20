// crates/ui/src/layout/spacer.rs

use gpui::*;

/// Flutter-style Spacer widget - takes up available space
#[derive(IntoElement)]
pub struct Spacer {
    flex: u32,
}

impl Spacer {
    pub fn new() -> Self {
        Self { flex: 1 }
    }

    pub fn flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Spacer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex_grow().flex_shrink_0()
    }
}
