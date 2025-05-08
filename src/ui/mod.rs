pub mod tui;
pub mod widgets;
pub mod theme;

// Re-export commonly used items
pub use widgets::{MenuItem, MenuType, Screen};
pub use theme::Theme;