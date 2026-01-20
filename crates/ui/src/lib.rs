// Core modules
pub mod accessibility;
pub mod color;
pub mod icons;
pub mod styled_ext;
pub mod theme;

// Component modules
pub mod components;

// Prelude for convenient imports
pub mod prelude;

use gpui::App;

/// Initialize the gpui-flutter library
pub fn init(cx: &mut gpui::AppContext) {
    // Register any global state or actions here
    theme::init(cx);
}
