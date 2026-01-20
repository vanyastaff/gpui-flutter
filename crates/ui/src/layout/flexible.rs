// crates/ui/src/layout/flexible.rs

use gpui::*;

/// Flutter-style Flexible widget
#[derive(IntoElement)]
pub struct Flexible {
    child: AnyElement,
    flex: u32,
    fit: FlexFit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexFit {
    /// Child can be smaller than flex space
    Loose,
    /// Child must fill all flex space
    Tight,
}

impl Flexible {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
            flex: 1,
            fit: FlexFit::Loose,
        }
    }

    pub fn flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }

    pub fn tight(mut self) -> Self {
        self.fit = FlexFit::Tight;
        self
    }

    pub fn loose(mut self) -> Self {
        self.fit = FlexFit::Loose;
        self
    }
}

impl RenderOnce for Flexible {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut element = div();

        // Set flex-grow value directly via style()
        element.style().flex_grow = Some(self.flex as f32);

        match self.fit {
            FlexFit::Loose => {
                // Loose: just flex-grow
                element.child(self.child)
            }
            FlexFit::Tight => {
                // Tight: flex-grow + flex-basis 0 + flex-shrink 1
                element.style().flex_basis = Some(px(0.).into());
                element.style().flex_shrink = Some(1.0);
                element.child(self.child)
            }
        }
    }
}

/// Flutter-style Expanded widget (Flexible with tight fit)
#[derive(IntoElement)]
pub struct Expanded {
    child: AnyElement,
    flex: u32,
}

impl Expanded {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
            flex: 1,
        }
    }

    pub fn flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }
}

impl RenderOnce for Expanded {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut element = div();

        // Set flex properties directly
        element.style().flex_grow = Some(self.flex as f32);
        element.style().flex_shrink = Some(1.0);
        element.style().flex_basis = Some(px(0.).into());

        element.child(self.child)
    }
}
