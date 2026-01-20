// Core modules
pub mod accessibility;
pub mod color;
pub mod icons;
pub mod styled_ext;
pub mod theme;

// Layout primitives
pub mod layout;

// Component modules
pub mod components;

// Prelude for convenient imports
pub mod prelude;

/// Initialize the gpui-flutter library
pub fn init() {
    // Register any global state or actions here
    theme::init();
}
