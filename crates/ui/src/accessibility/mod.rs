// crates/ui/src/accessibility/mod.rs

use gpui::{SharedString, Styled};

#[derive(Clone, Default)]
pub struct AccessibilityProps {
    pub role: Option<AriaRole>,
    pub label: Option<SharedString>,
    pub description: Option<SharedString>,
    pub hidden: bool,
    pub expanded: Option<bool>,
    pub selected: Option<bool>,
    pub disabled: bool,
    pub required: bool,
    pub invalid: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaRole {
    Button,
    Textbox,
    Checkbox,
    Radio,
    Dialog,
    Alert,
    Navigation,
    Main,
}

/// Trait for applying accessibility to elements
pub trait Accessible: Styled + Sized {
    fn accessibility(self, _props: AccessibilityProps) -> Self {
        // GPUI's accessibility will be implemented here
        // For now, this is a placeholder
        self
    }

    fn aria_label(self, label: impl Into<SharedString>) -> Self {
        let mut props = AccessibilityProps::default();
        props.label = Some(label.into());
        self.accessibility(props)
    }

    fn aria_hidden(self, hidden: bool) -> Self {
        let mut props = AccessibilityProps::default();
        props.hidden = hidden;
        self.accessibility(props)
    }

    fn aria_disabled(self, disabled: bool) -> Self {
        let mut props = AccessibilityProps::default();
        props.disabled = disabled;
        self.accessibility(props)
    }
}

// Implement for Div
impl Accessible for gpui::Div {}
