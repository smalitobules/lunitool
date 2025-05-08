/// Type of menu item
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuType {
    /// Simple text menu item
    Simple,
    /// Card-style menu item with border
    Card,
}

/// Menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// Unique identifier
    pub id: String,
    /// Display title
    pub title: String,
    /// Description (for card-style menus)
    pub description: String,
    /// Menu item type
    pub menu_type: MenuType,
}

/// Different screens in the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    /// Main menu
    MainMenu,
    /// Language selection
    LanguageSelect,
    /// Keyboard layout selection
    KeyboardSelect,
    /// Message dialog
    Message,
    /// Exit confirmation
    ConfirmExit,
}