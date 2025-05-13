/// Type of menu item
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuType {
    Simple,
    Card,
}

/// Menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub menu_type: MenuType,
}

/// Different screens in the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    MainMenu,
    LanguageSelect,
    KeyboardSelect,
    SystemInstallation,
    Message,
    ConfirmExit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DialogType {
    YesNo {
        title_key: String,
        message_key: String,
    },
    ThemeSelector,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisplayItemType {
    Disk,
    Partition,
    LuksContainer,
    LuksMappedContent,
    LvmVolumeGroup,
    LvmLogicalVolume,
    LvmPhysicalVolume,
    VeraCryptContainer,
    UnallocatedSpace, 
    FileSystemItem, 
    Label, 
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DisplayListItem {
    pub id_path: String,
    pub display_text: String,
    pub indent_level: usize,
    pub item_type: DisplayItemType,
    pub selectable: bool,
    pub size_bytes: Option<u64>,
}