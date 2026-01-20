// crates/ui/src/layout/center.rs

use gpui::*;

/// Flutter-style Center widget (shorthand for Align::center())
#[derive(IntoElement)]
pub struct Center {
    child: AnyElement,
}

impl Center {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
        }
    }
}

impl RenderOnce for Center {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h_full()
            .justify_center()
            .items_center()
            .child(self.child)
    }
}
