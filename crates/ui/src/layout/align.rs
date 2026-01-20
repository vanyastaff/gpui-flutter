// crates/ui/src/layout/align.rs

use super::Alignment;
use gpui::*;

/// Flutter-style Align widget
#[derive(IntoElement)]
pub struct Align {
    child: AnyElement,
    alignment: Alignment,
}

impl Align {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
            alignment: Alignment::Center,
        }
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    // Shortcut methods
    pub fn top_left(self) -> Self {
        self.alignment(Alignment::TopLeft)
    }

    pub fn top_center(self) -> Self {
        self.alignment(Alignment::TopCenter)
    }

    pub fn top_right(self) -> Self {
        self.alignment(Alignment::TopRight)
    }

    pub fn center_left(self) -> Self {
        self.alignment(Alignment::CenterLeft)
    }

    pub fn center(self) -> Self {
        self.alignment(Alignment::Center)
    }

    pub fn center_right(self) -> Self {
        self.alignment(Alignment::CenterRight)
    }

    pub fn bottom_left(self) -> Self {
        self.alignment(Alignment::BottomLeft)
    }

    pub fn bottom_center(self) -> Self {
        self.alignment(Alignment::BottomCenter)
    }

    pub fn bottom_right(self) -> Self {
        self.alignment(Alignment::BottomRight)
    }
}

impl RenderOnce for Align {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let element = div().flex().w_full().h_full();

        let element = match self.alignment {
            Alignment::TopLeft => element.justify_start().items_start(),
            Alignment::TopCenter => element.justify_center().items_start(),
            Alignment::TopRight => element.justify_end().items_start(),
            Alignment::CenterLeft => element.justify_start().items_center(),
            Alignment::Center => element.justify_center().items_center(),
            Alignment::CenterRight => element.justify_end().items_center(),
            Alignment::BottomLeft => element.justify_start().items_end(),
            Alignment::BottomCenter => element.justify_center().items_end(),
            Alignment::BottomRight => element.justify_end().items_end(),
        };

        element.child(self.child)
    }
}
