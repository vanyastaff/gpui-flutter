// crates/ui/src/icons/mod.rs

use crate::theme::ActiveTheme;
use gpui::*;
pub use lucide_icons::Icon as LucideIcon;

#[derive(IntoElement, Clone)]
pub struct Icon {
    icon: LucideIcon,
    size: IconSize,
    color: Option<Hsla>,

    // Accessibility
    label: Option<SharedString>,
    decorative: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum IconSize {
    Xs, // 12px
    Sm, // 16px
    Md, // 20px
    Lg, // 24px
    Xl, // 32px
}

impl IconSize {
    pub fn to_pixels(&self) -> Pixels {
        match self {
            IconSize::Xs => px(12.),
            IconSize::Sm => px(16.),
            IconSize::Md => px(20.),
            IconSize::Lg => px(24.),
            IconSize::Xl => px(32.),
        }
    }
}

impl Icon {
    pub fn new(icon: LucideIcon) -> Self {
        Self {
            icon,
            size: IconSize::Lg,
            color: None,
            label: None,
            decorative: false,
        }
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self.decorative = false;
        self
    }

    pub fn decorative(mut self) -> Self {
        self.decorative = true;
        self.label = None;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let color = self.color.unwrap_or(theme.colors.foreground.into());
        let size = self.size.to_pixels();

        // For now, render as a simple colored box
        // TODO: Implement actual SVG rendering when GPUI supports it
        div().size(size).bg(color).rounded(px(2.)).flex_none()
    }
}
