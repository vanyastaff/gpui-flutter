// crates/ui/src/styled_ext.rs

use gpui::{px, Styled};

/// Extension trait for convenient styling methods
pub trait StyledExt: Styled + Sized {
    // Layout helpers
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    // Gap utilities
    fn gap_1(self) -> Self {
        self.gap(px(4.))
    }

    fn gap_2(self) -> Self {
        self.gap(px(8.))
    }

    fn gap_3(self) -> Self {
        self.gap(px(12.))
    }

    fn gap_4(self) -> Self {
        self.gap(px(16.))
    }

    fn gap_6(self) -> Self {
        self.gap(px(24.))
    }

    fn gap_8(self) -> Self {
        self.gap(px(32.))
    }

    // Padding utilities
    fn p_1(self) -> Self {
        self.p(px(4.))
    }

    fn p_2(self) -> Self {
        self.p(px(8.))
    }

    fn p_3(self) -> Self {
        self.p(px(12.))
    }

    fn p_4(self) -> Self {
        self.p(px(16.))
    }

    fn p_6(self) -> Self {
        self.p(px(24.))
    }

    fn p_8(self) -> Self {
        self.p(px(32.))
    }

    fn px_1(self) -> Self {
        self.px(px(4.))
    }

    fn px_2(self) -> Self {
        self.px(px(8.))
    }

    fn px_3(self) -> Self {
        self.px(px(12.))
    }

    fn px_4(self) -> Self {
        self.px(px(16.))
    }

    fn py_1(self) -> Self {
        self.py(px(4.))
    }

    fn py_2(self) -> Self {
        self.py(px(8.))
    }

    fn py_3(self) -> Self {
        self.py(px(12.))
    }

    fn py_4(self) -> Self {
        self.py(px(16.))
    }

    // Border radius utilities
    fn rounded_sm(self) -> Self {
        self.rounded(px(4.))
    }

    fn rounded_md(self) -> Self {
        self.rounded(px(8.))
    }

    fn rounded_lg(self) -> Self {
        self.rounded(px(12.))
    }

    fn rounded_full(self) -> Self {
        self.rounded(px(9999.))
    }
}

// Implement for all Styled types
impl<T: Styled> StyledExt for T {}
