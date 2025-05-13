use ratatui::style::{Color, Style};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeName {
    AbletonDisco,
    BrownSugar,
    LadyLike,
    MateriaMatter,
    RatatuiRules,
    TerminalSpirit,
    UbuntuJoy,
    WhiteSur,
}

impl ThemeName {
    pub fn all() -> Vec<ThemeName> {
        vec![
            ThemeName::AbletonDisco,
            ThemeName::BrownSugar,
            ThemeName::LadyLike,
            ThemeName::MateriaMatter,
            ThemeName::RatatuiRules,
            ThemeName::TerminalSpirit,
            ThemeName::UbuntuJoy,
            ThemeName::WhiteSur,
        ]
    }
}

impl fmt::Display for ThemeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeName::AbletonDisco => write!(f, "Ableton Disco"),
            ThemeName::BrownSugar => write!(f, "Brown Sugar"),
            ThemeName::LadyLike => write!(f, "Lady Like"),
            ThemeName::MateriaMatter => write!(f, "Materia Matter"),
            ThemeName::RatatuiRules => write!(f, "Ratatui Rules"),
            ThemeName::TerminalSpirit => write!(f, "Terminal Spirit"),
            ThemeName::UbuntuJoy => write!(f, "Ubuntu Joy"),
            ThemeName::WhiteSur => write!(f, "White Sur"),
        }
    }
}

/// UI theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background_primary: Color,
    pub background_secondary: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub accent_primary: Color,
    pub accent_secondary: Color,
    pub border_primary: Color,
    pub border_highlight: Color,
    pub button_bg: Color,
    pub button_fg: Color,
    pub button_border: Color,
    pub dialog_bg: Color,
    pub dialog_fg: Color,
    pub dialog_border: Color,
    pub list_item_bg: Color,
    pub list_item_fg: Color,
    pub list_item_selected_bg: Color,
    pub list_item_selected_fg: Color,
    pub scrollbar_bg: Color,
    pub scrollbar_thumb: Color,
    pub app_bg: Color,
    pub content_bg: Color,
    pub block_bg: Color,
    pub text: Color,
    pub title: Color,
    pub border: Color,
    pub accent: Color,
    pub list_item_inactive_selection_bg: Color,
    pub list_item_inactive_selection_text: Color,
    pub scrollbar_fg: Color,
    pub input_text: Color,
    pub input_bg: Color,
    pub input_border: Color,
    pub input_border_focus: Color,
    pub button_hover_bg: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub log_panel_bg: Color,
    pub log_panel_text: Color,
    pub status_active: Option<Color>,
    pub status_completed: Option<Color>,
    pub status_pending: Option<Color>,
    pub status_failed: Option<Color>,
    pub dialog_title: Color,
    pub dialog_selected_option_bg: Color,
    pub dialog_selected_option_text: Color,
    pub header_module_text: Color,
    pub footer_text: Color,
    pub text_on_accent: Option<Color>,
    pub text_inverted: Option<Color>,
    pub gauge_label_on_filled: Option<Color>,
    pub gauge_label_on_empty: Option<Color>,
}

impl Default for Theme {
    fn default() -> Self {
        // The default theme is now Terminal Spirit
        Self::terminal_spirit()
    }
}

impl Theme {
    /// Get a theme by name
    pub fn get(name: &str) -> Self {
        match name {
            "Ableton Disco" => Self::ableton_disco(),
            "Brown Sugar" => Self::brown_sugar(),
            "Lady Like" => Self::lady_like(),
            "Materia Matter" => Self::materia_matter(),
            "Ratatui Rules" => Self::ratatui_rules(),
            "Terminal Spirit" => Self::terminal_spirit(),
            "Ubuntu Joy" => Self::ubuntu_joy(),
            "White Sur" => Self::white_sur(),
            _ => Self::default(), // Fallback to default (Terminal Spirit)
        }
    }

    /// Original Ratatui theme definition renamed
    pub fn ratatui_rules() -> Self {
        let app_bg = Color::Rgb(20, 20, 40);
        let content_bg = Color::Rgb(30, 30, 50);
        let text_color = Color::Rgb(220, 220, 240);
        let text_secondary_color = Color::Rgb(180, 180, 200);
        let title_color = Color::LightCyan;
        let accent_color = Color::Yellow;
        let border_color = Color::Rgb(80, 80, 110);
        let header_module_text_color = Color::Rgb(255, 165, 0);
        let footer_text_color = Color::Rgb(160, 160, 180);
        let list_item_selected_bg = Color::Rgb(70, 70, 100);
        let list_item_selected_text = Color::Rgb(250, 250, 250);
        let dialog_bg_color = Color::Rgb(50, 50, 80);
        let dialog_text_color = Color::Rgb(220, 220, 240);
        let dialog_border_color = Color::Rgb(100, 100, 140);
        let dialog_title_color = Color::Yellow;
        let dialog_selected_option_bg = Color::Rgb(90, 90, 120);
        let dialog_selected_option_text = Color::Rgb(250, 250, 250);
        let log_panel_bg = Color::Rgb(20, 20, 30);
        let scrollbar_bg = Color::Rgb(45, 45, 65);
        let scrollbar_thumb = Color::Rgb(100, 100, 130);

        Theme {
            name: ThemeName::RatatuiRules.to_string(),
            background_primary: app_bg,
            background_secondary: content_bg,
            text_primary: text_color,
            text_secondary: text_secondary_color,
            accent_primary: title_color,
            accent_secondary: accent_color,
            border_primary: border_color,
            border_highlight: Color::Cyan,
            app_bg: app_bg,
            content_bg: content_bg,
            block_bg: content_bg,
            text: text_color,
            title: title_color,
            border: border_color,
            accent: accent_color,
            list_item_fg: text_color,
            list_item_selected_fg: list_item_selected_text,
            list_item_bg: content_bg,
            list_item_selected_bg: list_item_selected_bg,
            log_panel_bg: log_panel_bg,
            log_panel_text: text_color,
            scrollbar_bg: scrollbar_bg,
            scrollbar_thumb: scrollbar_thumb,
            status_active: Some(accent_color),
            status_completed: Some(Color::Green),
            status_pending: Some(text_secondary_color),
            status_failed: Some(Color::Red),
            dialog_bg: dialog_bg_color,
            dialog_fg: dialog_text_color,
            dialog_border: dialog_border_color,
            dialog_title: dialog_title_color,
            dialog_selected_option_bg: dialog_selected_option_bg,
            dialog_selected_option_text: dialog_selected_option_text,
            input_bg: content_bg,
            input_text: text_color,
            input_border: border_color,
            input_border_focus: border_color,
            button_bg: accent_color,
            button_fg: text_color,
            button_border: accent_color,
            success: Color::Rgb(70, 180, 70),
            warning: Color::Rgb(220, 180, 0),
            error: Color::Rgb(200, 70, 70),
            info: Color::Rgb(70, 170, 220),
            header_module_text: header_module_text_color,
            footer_text: footer_text_color,
            list_item_inactive_selection_bg: list_item_selected_bg,
            list_item_inactive_selection_text: list_item_selected_text,
            scrollbar_fg: border_color,
            text_on_accent: Some(Color::Black),
            text_inverted: Some(Color::Black),
            gauge_label_on_filled: Some(Color::Black),
            gauge_label_on_empty: Some(Color::White),
            button_hover_bg: accent_color,
        }
    }

    /// Light theme
    pub fn white_sur() -> Self {
        let app_bg_color = Color::Rgb(230, 230, 235);
        let content_bg_color = Color::Rgb(245, 245, 250);
        let block_bg_color = Color::Rgb(250, 250, 255);
        let text_color = Color::Rgb(20, 20, 25);
        let text_secondary_color = Color::Rgb(80, 80, 90);
        let title_color = Color::Rgb(10, 10, 15);
        let border_color = Color::Rgb(200, 200, 205);
        let accent_color = Color::Rgb(0, 122, 255);
        let accent_secondary_color = Color::Rgb(255, 149, 0);
        let list_item_text_color = Color::Rgb(40, 40, 45);
        let list_item_selected_text_color = Color::Rgb(30, 30, 30);
        let list_item_selected_bg_color = Color::Rgb(170, 210, 255);
        let header_module_text_color = Color::Rgb(255, 59, 48);

        Theme {
            name: ThemeName::WhiteSur.to_string(),
            background_primary: app_bg_color,
            background_secondary: content_bg_color,
            text_primary: text_color,
            text_secondary: text_secondary_color,
            accent_primary: accent_color,
            accent_secondary: accent_secondary_color,
            border_primary: border_color,
            border_highlight: accent_color,
            app_bg: app_bg_color,
            content_bg: content_bg_color,
            block_bg: block_bg_color,
            text: text_color,
            title: title_color,
            border: border_color,
            accent: accent_color,
            list_item_fg: list_item_text_color,
            list_item_selected_fg: list_item_selected_text_color,
            list_item_bg: block_bg_color,
            list_item_selected_bg: list_item_selected_bg_color,
            list_item_inactive_selection_bg: Color::Rgb(210, 210, 215),
            list_item_inactive_selection_text: Color::Rgb(100, 100, 105),
            scrollbar_fg: Color::Rgb(180, 180, 185),
            scrollbar_bg: Color::Rgb(230, 230, 235),
            scrollbar_thumb: Color::Rgb(150, 150, 155),
            input_text: text_color,
            input_bg: Color::White,
            input_border: Color::Rgb(190, 190, 195),
            input_border_focus: accent_color,
            button_fg: Color::White,
            button_bg: accent_color,
            button_hover_bg: accent_color,
            button_border: accent_color,
            success: Color::Rgb(40, 167, 69),
            warning: Color::Rgb(255, 193, 7),
            error: Color::Rgb(220, 53, 69),
            info: Color::Rgb(23, 162, 184),
            log_panel_bg: Color::Rgb(220, 220, 225),
            log_panel_text: Color::Rgb(30, 30, 35),
            status_active: Some(Color::Rgb(0, 122, 255)),
            status_completed: Some(Color::Rgb(52, 199, 89)),
            status_pending: Some(Color::Rgb(120, 120, 128)),
            status_failed: Some(Color::Rgb(255, 59, 48)),
            dialog_bg: Color::Rgb(240, 240, 245),
            dialog_fg: Color::Rgb(20, 20, 20),
            dialog_border: Color::Rgb(180, 180, 180),
            dialog_title: Color::Rgb(10, 10, 10),
            dialog_selected_option_bg: Color::Rgb(0, 122, 255),
            dialog_selected_option_text: Color::White,
            header_module_text: header_module_text_color,
            footer_text: text_secondary_color,
            text_on_accent: Some(Color::White),
            text_inverted: Some(Color::Rgb(30,30,30)),
            gauge_label_on_filled: Some(Color::White),
            gauge_label_on_empty: Some(Color::Rgb(20, 20, 25)),
        }
    }

    /// Terminal Theme (inspired by screenshot) - This is also the default theme, renamed
    pub fn terminal_spirit() -> Self {
        let app_bg_color = Color::Black;
        let content_bg_color = Color::Rgb(15, 15, 15);
        let text_color = Color::Rgb(210, 210, 210);
        let text_secondary_color = Color::Rgb(160, 160, 160);
        let title_color = Color::Rgb(0, 200, 0);
        let border_color = Color::Rgb(80, 80, 80);
        let accent_color = Color::Yellow;
        let border_highlight_color = Color::White;
        let list_item_fg = text_color;
        let list_item_selected_fg = accent_color;
        let list_item_selected_bg_color = Color::Rgb(30, 30, 30);
        let header_module_text_color = title_color;
        let footer_text_color = text_secondary_color;

        Theme {
            name: ThemeName::TerminalSpirit.to_string(),
            background_primary: app_bg_color,
            background_secondary: content_bg_color,
            text_primary: text_color,
            text_secondary: text_secondary_color,
            accent_primary: title_color,
            accent_secondary: accent_color,
            border_primary: border_color,
            border_highlight: border_highlight_color,
            app_bg: app_bg_color,
            content_bg: content_bg_color,
            block_bg: content_bg_color,
            text: text_color,
            title: title_color,
            border: border_color,
            accent: accent_color,
            list_item_fg: list_item_fg,
            list_item_selected_fg: list_item_selected_fg,
            list_item_bg: content_bg_color,
            list_item_selected_bg: list_item_selected_bg_color,
            list_item_inactive_selection_bg: list_item_selected_bg_color,
            list_item_inactive_selection_text: Color::DarkGray,
            scrollbar_fg: border_color,
            scrollbar_bg: app_bg_color,
            scrollbar_thumb: accent_color,
            input_text: text_color,
            input_bg: content_bg_color,
            input_border: border_color,
            input_border_focus: accent_color,
            button_fg: Color::Black,
            button_bg: accent_color,
            button_hover_bg: Color::Rgb(255, 255, 100),
            button_border: accent_color,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            log_panel_bg: Color::Rgb(5, 5, 5),
            log_panel_text: text_secondary_color,
            status_active: Some(accent_color),
            status_completed: Some(Color::Green),
            status_pending: Some(text_secondary_color),
            status_failed: Some(Color::Red),
            dialog_bg: Color::Rgb(25, 25, 25),
            dialog_fg: text_color,
            dialog_border: accent_color,
            dialog_title: title_color,
            dialog_selected_option_bg: accent_color,
            dialog_selected_option_text: Color::Black,
            header_module_text: header_module_text_color,
            footer_text: footer_text_color,
            text_on_accent: Some(Color::Black),
            text_inverted: Some(Color::Black),
            gauge_label_on_filled: Some(Color::Black),
            gauge_label_on_empty: Some(Color::White),
        }
    }

    /// Ableton Disco theme
    pub fn ableton_disco() -> Self {
        let base_bg = Color::Rgb(18, 18, 18);
        let content_bg = Color::Rgb(40, 40, 40);
        let block_bg_alt = Color::Rgb(55, 55, 55);
        let text_normal = Color::Rgb(210, 210, 210);
        let text_secondary = Color::Rgb(150, 150, 150);
        let accent_orange = Color::Rgb(255, 130, 0);
        let accent_yellow = Color::Rgb(240, 220, 100);
        let accent_green = Color::Rgb(80, 180, 90);
        let accent_blue_light = Color::Rgb(100, 180, 220);
        let border_color = Color::Rgb(60, 60, 60);

        Self {
            name: ThemeName::AbletonDisco.to_string(),
            background_primary: base_bg,
            background_secondary: content_bg,
            text_primary: text_normal,
            text_secondary: text_secondary,
            accent_primary: accent_orange,
            accent_secondary: accent_green,
            border_primary: border_color,
            border_highlight: accent_yellow,
            app_bg: base_bg,
            content_bg: content_bg,
            block_bg: content_bg,
            text: text_normal,
            title: accent_orange,
            border: border_color,
            accent: accent_orange,
            list_item_fg: text_normal,
            list_item_selected_fg: Color::Black,
            list_item_bg: content_bg,
            list_item_selected_bg: accent_yellow,
            list_item_inactive_selection_bg: accent_yellow,
            list_item_inactive_selection_text: Color::Rgb(50,50,50),
            scrollbar_fg: accent_orange,
            scrollbar_bg: content_bg,
            scrollbar_thumb: accent_yellow,
            input_text: text_normal,
            input_bg: base_bg,
            input_border: accent_blue_light,
            input_border_focus: accent_orange,
            button_fg: Color::Black,
            button_bg: accent_yellow,
            button_hover_bg: accent_yellow,
            button_border: accent_orange,
            success: accent_green,
            warning: accent_orange,
            error: Color::Rgb(220, 50, 50),
            info: accent_blue_light,
            log_panel_bg: block_bg_alt,
            log_panel_text: text_secondary,
            status_active: Some(accent_orange),
            status_completed: Some(accent_green),
            status_pending: Some(text_secondary),
            status_failed: Some(Color::Rgb(220, 50, 50)),
            dialog_bg: Color::Rgb(45, 45, 45),
            dialog_fg: text_normal,
            dialog_border: accent_blue_light,
            dialog_title: accent_orange,
            dialog_selected_option_bg: accent_yellow,
            dialog_selected_option_text: Color::Rgb(20,20,20),
            header_module_text: accent_green,
            footer_text: text_secondary,
            text_on_accent: Some(Color::Black),
            text_inverted: Some(Color::Rgb(18,18,18)),
            gauge_label_on_filled: Some(Color::Black),
            gauge_label_on_empty: Some(Color::Rgb(210, 210, 210)),
        }
    }

    /// Ubuntu theme, renamed
    pub fn ubuntu_joy() -> Self {
        let base_bg = Color::Rgb(30, 30, 30);
        let content_bg = Color::Rgb(38, 38, 38);
        let text_normal = Color::Rgb(230, 230, 230);
        let text_secondary = Color::Rgb(160, 160, 160);
        let accent_ubuntu_orange = Color::Rgb(233, 84, 32);
        let accent_ubuntu_aubergine = Color::Rgb(140, 56, 118);
        let border_color = Color::Rgb(55, 55, 55);
        let selected_bg = accent_ubuntu_orange;

        Self {
            name: ThemeName::UbuntuJoy.to_string(),
            background_primary: base_bg,
            background_secondary: content_bg,
            text_primary: text_normal,
            text_secondary: text_secondary,
            accent_primary: accent_ubuntu_orange,
            accent_secondary: accent_ubuntu_aubergine,
            border_primary: border_color,
            border_highlight: accent_ubuntu_orange,
            app_bg: base_bg,
            content_bg: content_bg,
            block_bg: content_bg,
            text: text_normal,
            title: accent_ubuntu_orange,
            border: border_color,
            accent: accent_ubuntu_orange,
            list_item_fg: text_normal,
            list_item_selected_fg: Color::White,
            list_item_bg: content_bg,
            list_item_selected_bg: selected_bg,
            list_item_inactive_selection_bg: selected_bg,
            list_item_inactive_selection_text: Color::Rgb(180,180,180),
            scrollbar_fg: accent_ubuntu_orange,
            scrollbar_bg: content_bg,
            scrollbar_thumb: accent_ubuntu_orange,
            input_text: text_normal,
            input_bg: base_bg,
            input_border: border_color,
            input_border_focus: accent_ubuntu_orange,
            button_fg: Color::White,
            button_bg: accent_ubuntu_orange,
            button_hover_bg: accent_ubuntu_orange,
            button_border: accent_ubuntu_orange,
            success: Color::Rgb(46, 204, 113),
            warning: Color::Rgb(241, 196, 15),
            error: Color::Rgb(231, 76, 60),
            info: accent_ubuntu_orange,
            log_panel_bg: base_bg,
            log_panel_text: text_secondary,
            status_active: Some(accent_ubuntu_aubergine),
            status_completed: Some(Color::Rgb(46, 204, 113)),
            status_pending: Some(text_secondary),
            status_failed: Some(Color::Rgb(231, 76, 60)),
            dialog_bg: Color::Rgb(45, 45, 45),
            dialog_fg: text_normal,
            dialog_border: accent_ubuntu_orange,
            dialog_title: accent_ubuntu_orange,
            dialog_selected_option_bg: accent_ubuntu_orange,
            dialog_selected_option_text: Color::White,
            header_module_text: accent_ubuntu_aubergine,
            footer_text: text_secondary,
            text_on_accent: Some(Color::White),
            text_inverted: Some(Color::Black),
            gauge_label_on_filled: Some(Color::White),
            gauge_label_on_empty: Some(Color::Rgb(230, 230, 230)),
        }
    }

    /// Materia Dark theme (inspired by Materia KDE), renamed
    pub fn materia_matter() -> Self {
        let app_bg_color = Color::Rgb(27, 27, 27);
        let content_bg_color = Color::Rgb(37, 37, 37);
        let block_bg_color = Color::Rgb(47, 47, 47);
        let text_color = Color::Rgb(238, 238, 238);
        let text_secondary_color = Color::Rgb(189, 189, 189);
        let title_color = Color::Rgb(130, 170, 255);
        let border_color = Color::Rgb(67, 67, 67);
        let accent_color = Color::Rgb(130, 170, 255);
        let accent_secondary_color = Color::Rgb(37, 197, 218);
        let list_item_fg = text_color;
        let list_item_selected_fg = Color::White;
        let list_item_selected_bg_color = Color::Rgb(60, 80, 115);
        let header_module_text_color = Color::Rgb(37, 197, 218);
        let footer_text_color = text_secondary_color;

        Theme {
            name: ThemeName::MateriaMatter.to_string(),
            background_primary: app_bg_color,
            background_secondary: content_bg_color,
            text_primary: text_color,
            text_secondary: text_secondary_color,
            accent_primary: accent_color,
            accent_secondary: accent_secondary_color,
            border_primary: border_color,
            border_highlight: accent_color,
            app_bg: app_bg_color,
            content_bg: content_bg_color,
            block_bg: content_bg_color,
            text: text_color,
            title: title_color,
            border: border_color,
            accent: accent_color,
            list_item_fg: list_item_fg,
            list_item_selected_fg: list_item_selected_fg,
            list_item_bg: content_bg_color,
            list_item_selected_bg: list_item_selected_bg_color,
            list_item_inactive_selection_bg: Color::Rgb(50, 60, 80),
            list_item_inactive_selection_text: Color::Rgb(150, 160, 180),
            scrollbar_fg: accent_color,
            scrollbar_bg: block_bg_color,
            scrollbar_thumb: accent_color,
            input_text: text_color,
            input_bg: content_bg_color,
            input_border: border_color,
            input_border_focus: accent_color,
            button_fg: Color::White,
            button_bg: accent_color,
            button_hover_bg: accent_color,
            button_border: accent_color,
            success: Color::Rgb(76, 175, 80),
            warning: Color::Rgb(255, 152, 0),
            error: Color::Rgb(244, 67, 54),
            info: Color::Rgb(33, 150, 243),
            log_panel_bg: Color::Rgb(20, 20, 20),
            log_panel_text: Color::Rgb(210, 210, 210),
            status_active: Some(accent_secondary_color),
            status_completed: Some(Color::Rgb(76, 175, 80)),
            status_pending: Some(Color::Gray),
            status_failed: Some(Color::Rgb(244, 67, 54)),
            dialog_bg: Color::Rgb(57, 57, 57),
            dialog_fg: text_color,
            dialog_border: accent_color,
            dialog_title: title_color,
            dialog_selected_option_bg: accent_color,
            dialog_selected_option_text: Color::White,
            header_module_text: header_module_text_color,
            footer_text: footer_text_color,
            text_on_accent: Some(Color::Black),
            text_inverted: Some(Color::White),
            gauge_label_on_filled: Some(Color::Black),
            gauge_label_on_empty: Some(Color::Rgb(238, 238, 238)),
        }
    }

    /// Lady Like Theme
    pub fn lady_like() -> Self {
        let app_bg_color = Color::Rgb(40, 30, 45);
        let content_bg_color = Color::Rgb(50, 40, 55);
        let text_color = Color::Rgb(230, 220, 235);
        let text_secondary_color = Color::Rgb(180, 170, 185);
        let title_color = Color::Rgb(255, 105, 180);
        let border_color = Color::Rgb(90, 80, 95);
        let border_highlight_color = Color::Rgb(255, 20, 147);
        let accent_color = Color::Rgb(218, 112, 214);
        let accent_secondary_color = Color::Rgb(173, 216, 230);
        let list_item_fg = text_color;
        let list_item_selected_fg = Color::Rgb(30, 20, 35);
        let list_item_selected_bg_color = accent_color;
        let header_module_text_color = Color::Rgb(255, 182, 193);
        let footer_text_color = text_secondary_color;

        Theme {
            name: ThemeName::LadyLike.to_string(),
            background_primary: app_bg_color,
            background_secondary: content_bg_color,
            text_primary: text_color,
            text_secondary: text_secondary_color,
            accent_primary: title_color,
            accent_secondary: accent_secondary_color,
            border_primary: border_color,
            border_highlight: border_highlight_color,
            app_bg: app_bg_color,
            content_bg: content_bg_color,
            block_bg: content_bg_color,
            text: text_color,
            title: title_color,
            border: border_color,
            accent: accent_color,
            list_item_fg: list_item_fg,
            list_item_selected_fg: list_item_selected_fg,
            list_item_bg: content_bg_color,
            list_item_selected_bg: list_item_selected_bg_color,
            list_item_inactive_selection_bg: Color::Rgb(70, 60, 75),
            list_item_inactive_selection_text: Color::Rgb(140, 130, 145),
            scrollbar_fg: Color::Rgb(100, 90, 105),
            scrollbar_bg: Color::Rgb(40, 30, 45),
            scrollbar_thumb: accent_color,
            input_text: text_color,
            input_bg: content_bg_color,
            input_border: border_color,
            input_border_focus: border_highlight_color,
            button_fg: Color::Rgb(30, 20, 35),
            button_bg: accent_color,
            button_hover_bg: accent_color,
            button_border: accent_color,
            success: Color::Rgb(144, 238, 144),
            warning: Color::Rgb(255, 223, 186),
            error: Color::Rgb(250, 128, 114),
            info: Color::Rgb(175, 238, 238),
            log_panel_bg: Color::Rgb(30, 20, 35),
            log_panel_text: Color::Rgb(210, 200, 215),
            status_active: Some(accent_secondary_color),
            status_completed: Some(Color::Rgb(152, 251, 152)),
            status_pending: Some(Color::DarkGray),
            status_failed: Some(Color::Rgb(240, 128, 128)),
            dialog_bg: Color::Rgb(70, 60, 75),
            dialog_fg: text_color,
            dialog_border: border_highlight_color,
            dialog_title: title_color,
            dialog_selected_option_bg: accent_color,
            dialog_selected_option_text: Color::Rgb(30, 20, 35),
            header_module_text: header_module_text_color,
            footer_text: footer_text_color,
            text_on_accent: Some(Color::Black),
            text_inverted: Some(Color::Black),
            gauge_label_on_filled: Some(Color::Black),
            gauge_label_on_empty: Some(Color::Rgb(230, 220, 235)),
        }
    }

    /// Brown Sugar Theme
    pub fn brown_sugar() -> Self {
        let app_bg_color = Color::Rgb(35, 20, 15);
        let content_bg_color = Color::Rgb(65, 45, 35);
        let text_color = Color::Rgb(255, 250, 240);
        let text_secondary_color = Color::Rgb(160, 145, 130);
        let title_color = Color::Rgb(240, 140, 50);
        let border_color = Color::Rgb(85, 60, 45);
        let border_highlight_color = Color::Rgb(255, 165, 70);
        let accent_color = Color::Rgb(245, 170, 90);
        let accent_secondary_color = Color::Rgb(255, 200, 140);
        let list_item_fg = text_color;
        let list_item_selected_fg = Color::Rgb(30, 15, 10);
        let list_item_selected_bg_color = accent_color;
        let header_module_text_color = Color::Rgb(250, 210, 160);
        let footer_text_color = text_secondary_color;
        let button_text_color = Color::Rgb(30, 15, 10);

        Theme {
            name: ThemeName::BrownSugar.to_string(),
            background_primary: app_bg_color,
            background_secondary: content_bg_color,
            text_primary: text_color,
            text_secondary: text_secondary_color,
            accent_primary: accent_color,
            accent_secondary: accent_secondary_color,
            border_primary: border_color,
            border_highlight: border_highlight_color,
            app_bg: app_bg_color,
            content_bg: content_bg_color,
            block_bg: content_bg_color,
            text: text_color,
            title: title_color,
            border: border_color,
            accent: accent_color,
            list_item_fg: list_item_fg,
            list_item_selected_fg: list_item_selected_fg,
            list_item_bg: content_bg_color,
            list_item_selected_bg: list_item_selected_bg_color,
            list_item_inactive_selection_bg: Color::Rgb(70, 50, 40),
            list_item_inactive_selection_text: Color::Rgb(160, 145, 130),
            scrollbar_fg: Color::Rgb(100, 75, 60),
            scrollbar_bg: Color::Rgb(45, 30, 23),
            scrollbar_thumb: accent_color,
            input_text: text_color,
            input_bg: content_bg_color,
            input_border: border_color,
            input_border_focus: border_highlight_color,
            button_fg: button_text_color,
            button_bg: accent_color,
            button_hover_bg: accent_secondary_color,
            button_border: accent_color,
            success: Color::Rgb(124, 252, 0),
            warning: Color::Rgb(255, 165, 0),
            error: Color::Rgb(178, 34, 34),
            info: Color::Rgb(135, 206, 235),
            log_panel_bg: Color::Rgb(25, 15, 10),
            log_panel_text: text_secondary_color,
            status_active: Some(accent_secondary_color),
            status_completed: Some(Color::Rgb(124, 252, 0)),
            status_pending: Some(text_secondary_color),
            status_failed: Some(Color::Rgb(178, 34, 34)),
            dialog_bg: Color::Rgb(70, 50, 40),
            dialog_fg: text_color,
            dialog_border: border_highlight_color,
            dialog_title: title_color,
            dialog_selected_option_bg: accent_color,
            dialog_selected_option_text: button_text_color,
            header_module_text: header_module_text_color,
            footer_text: footer_text_color,
            text_on_accent: Some(button_text_color),
            text_inverted: Some(Color::White),
            gauge_label_on_filled: Some(Color::Rgb(40, 30, 20)),
            gauge_label_on_empty: Some(text_color),
        }
    }

    /// Get normal text style
    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text).bg(self.app_bg)
    }

    /// Get title style
    pub fn title_style(&self) -> Style {
        Style::default().fg(self.title).bg(self.content_bg)
    }

    /// Get highlight style
    pub fn highlight_style(&self) -> Style {
        Style::default().fg(self.accent)
    }

    /// Get secondary highlight style
    pub fn highlight_secondary_style(&self) -> Style {
        Style::default().fg(self.accent_secondary)
    }

    /// Get warning style
    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.warning).bg(self.app_bg)
    }

    /// Get error style
    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error).bg(self.app_bg)
    }

    /// Get app style
    pub fn app_style(&self) -> Style {
        Style::default().fg(self.text).bg(self.app_bg)
    }

    /// Get border style
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Get border highlight style
    pub fn border_highlight_style(&self) -> Style {
        Style::default().fg(self.border_highlight)
    }
}