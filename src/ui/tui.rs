use anyhow::{Context, Result};
use crossterm::{
    event::{EnableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen},
};
use std::io;
use ratatui::{
    backend::{CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect, Alignment, Margin},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, BorderType, List, ListItem, Paragraph, ListState, Wrap, Scrollbar, ScrollbarState, Clear, Gauge},
    Frame, Terminal,
};

use crate::{
    lang::get_text,
    ui::widgets::{Screen, DialogType, DisplayListItem, DisplayItemType, MenuItem},
    app::{InstallationStep, InstallationConfig, InstallationTaskItem, InstallationTaskStatus},
    core::disk_info::{SystemDiskInfo, PartitionContent, MappedContent},
};
use crate::ui::theme::Theme;
use crate::ui::theme::ThemeName;

/// UI state for the application
#[derive(Debug)]
pub struct UiState {
    pub current_screen: Screen,
    pub menu_items: Vec<MenuItem>,
    pub selected_index: usize,
    pub languages: Vec<String>,
    pub keyboards: Vec<String>,
    pub message: Option<(String, String)>,
    pub previous_screen: Screen,

    pub installation_step: Option<InstallationStep>,
    pub installation_config: InstallationConfig,
    pub installation_tasks: Vec<InstallationTaskItem>,
    pub current_installation_task_index: usize,
    pub show_log_panel: bool,
    pub input_buffer: String,
    pub installation_task_list_state: ListState,

    pub system_disk_info: Option<SystemDiskInfo>,
    pub disk_setup_selected_item_path: Option<String>,
    pub disk_setup_list_state: ListState,
    pub current_disk_display_items: Vec<DisplayListItem>,
    pub is_loading_disks: bool, 
    pub log_buffer: Vec<String>,

    // Scroll state for task description
    pub task_description_scroll_offset: usize,
    pub task_description_total_lines: usize,
    pub task_description_scrollbar_state: ScrollbarState,

    pub active_dialog: Option<DialogType>,
    pub dialog_selected_option: usize,

    pub themes: Vec<Theme>,
    pub active_theme_index: usize,
}

impl UiState {
    /// Create a new UI state
    pub fn new(menu_items: Vec<MenuItem>) -> Self {
        let mut initial_task_list_state = ListState::default();
        initial_task_list_state.select(Some(0));

        let disk_setup_list_state = ListState::default();

        let themes = ThemeName::all()
            .iter()
            .map(|name_enum| Theme::get(&name_enum.to_string()))
            .collect::<Vec<Theme>>();

        let active_theme_index = themes.iter().position(|theme| theme.name == "Terminal Spirit")
            .unwrap_or(0);

        Self {
            current_screen: Screen::LanguageSelect,
            menu_items,
            selected_index: 0,
            languages: vec!["de".to_string(), "en".to_string()],
            keyboards: vec!["de".to_string(), "us".to_string()],
            message: None,
            previous_screen: Screen::LanguageSelect,
            installation_step: None,
            installation_config: InstallationConfig::default(),
            installation_tasks: Vec::new(),
            current_installation_task_index: 0,
            show_log_panel: false,
            input_buffer: String::new(),
            installation_task_list_state: initial_task_list_state,
            system_disk_info: None,
            disk_setup_selected_item_path: None,
            disk_setup_list_state,
            current_disk_display_items: Vec::new(), 
            is_loading_disks: false, 
            log_buffer: Vec::new(),
            task_description_scroll_offset: 0,
            task_description_total_lines: 0,
            task_description_scrollbar_state: ScrollbarState::default(),
            active_dialog: None,
            dialog_selected_option: 0,
            themes,
            active_theme_index,
        }
    }

    /// Sets the current screen and resets selected_index if appropriate.
    pub fn set_current_screen(&mut self, new_screen: Screen) {
        self.previous_screen = self.current_screen;
        self.current_screen = new_screen;

        // Reset selected_index for screens with list/menu structures.
        match new_screen {
            Screen::MainMenu | 
            Screen::LanguageSelect | 
            Screen::KeyboardSelect | 
            Screen::ConfirmExit => {
                self.selected_index = 0;
            }
            Screen::Message => {
                // No reset for Message screen.
            }
            Screen::SystemInstallation => {
                // selected_index for SystemInstallation is managed by its own logic.
            }
        }
    }

    /// Get selected menu item
    pub fn selected_menu_item(&self) -> Option<&MenuItem> {
        self.menu_items.get(self.selected_index)
    }

    /// Select next menu item
    pub fn next_menu_item(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.menu_items.len();
    }

    /// Select previous menu item
    pub fn previous_menu_item(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.menu_items.len() - 1;
        }
    }

    /// Get selected language
    pub fn selected_language(&self) -> Option<String> {
        self.languages.get(self.selected_index).cloned()
    }

    /// Select next language
    pub fn next_language(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.languages.len();
    }

    /// Select previous language
    pub fn previous_language(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.languages.len() - 1;
        }
    }

    /// Get selected keyboard layout
    pub fn selected_keyboard(&self) -> Option<String> {
        self.keyboards.get(self.selected_index).cloned()
    }

    /// Select next keyboard layout
    pub fn next_keyboard(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.keyboards.len();
    }

    /// Select previous keyboard layout
    pub fn previous_keyboard(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.keyboards.len() - 1;
        }
    }

    /// Show a message dialog
    pub fn show_message(&mut self, title: &str, content: &str) {
        self.previous_screen = self.current_screen;
        self.message = Some((title.to_string(), content.to_string()));
        self.current_screen = Screen::Message;
    }

    /// Show an error dialog
    pub fn show_error(&mut self, title: &str, content: &str) {
        self.show_message(&format!("Error: {}", title), content);
    }

    /// Clear the current message
    pub fn clear_message(&mut self) {
        self.message = None;
        self.current_screen = self.previous_screen;
    }

    /// Helper function to determine if current installation step needs text input
    pub fn installation_step_requires_text_input(&self) -> bool {
        if let Some(step) = self.installation_step {
            matches!(step, InstallationStep::UserSetup)
        } else {
            false
        }
    }

    /// Gets the current installation task name
    pub fn current_installation_task_name(&self) -> String {
        self.installation_tasks
            .get(self.current_installation_task_index)
            .map_or_else(|| get_text("UNKNOWN_TASK"), |task| task.title.clone())
    }
}

/// Setup the terminal
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    terminal::enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).context("Failed to create terminal")?;
    Ok(terminal)
}

/// Draw the UI based on current state
pub fn draw_ui(f: &mut Frame, state: &mut UiState) {
    let active_theme_index = state.active_theme_index;

    if state.active_dialog.is_some() {
        // --- MODE: DIALOG ACTIVE ---
        let theme_clone_for_dialog = state.themes[active_theme_index].clone();

        f.render_widget(Clear, f.area());
        let overlay_bg_color = Color::Black;
        f.render_widget(Block::default().bg(overlay_bg_color), f.area());
        draw_active_dialog_popup(f, state, &theme_clone_for_dialog); 

    } else {
        // --- MODE: NO DIALOG ACTIVE (Normal UI) ---
        let current_theme_ref = &state.themes[active_theme_index];

        f.render_widget(Block::default().bg(current_theme_ref.background_secondary), f.area());
        let app_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(current_theme_ref.border_primary))
            .bg(current_theme_ref.background_secondary);
        f.render_widget(app_block.clone(), f.area()); 

        let content_area_within_borders = app_block.inner(f.area());
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), 
                Constraint::Min(0),    
                Constraint::Length(2), 
            ].as_ref())
            .split(content_area_within_borders);

        let header_area = main_chunks[0];
        let content_area = main_chunks[1];
        let footer_area = main_chunks[2];

        draw_header(f, state, header_area, current_theme_ref);

        match state.current_screen {
            Screen::MainMenu => draw_main_menu(f, state, content_area, current_theme_ref),
            Screen::LanguageSelect => draw_language_select(f, state, content_area, current_theme_ref),
            Screen::KeyboardSelect => draw_keyboard_select(f, state, content_area, current_theme_ref),
            Screen::SystemInstallation => {
                // Since draw_installation_screen takes state mutably, clone the theme beforehand.
                let theme_clone_for_installation = state.themes[active_theme_index].clone();
                draw_installation_screen(f, state, content_area, &theme_clone_for_installation);
            }
            Screen::Message => draw_message(f, state, content_area, current_theme_ref),
            Screen::ConfirmExit => draw_confirm_exit(f, state, content_area, current_theme_ref),
        }
        // The theme reference is re-fetched here to avoid potential conflicts with the borrow checker,
        // if `state` was borrowed mutably in the match block above.
        draw_footer(f, state, footer_area, &state.themes[active_theme_index]); 
    }
}

fn draw_header(f: &mut Frame, state: &UiState, area: Rect, theme: &Theme) { 
    let bg_color = theme.background_secondary;
    
    let logo_color = theme.accent_primary; 
    let main_title_color = theme.accent_primary;
    let breadcrumb_text_color = theme.accent_secondary; 
    let breadcrumb_separator_color = theme.accent_primary; 

    let header_block = Block::default().bg(bg_color);
    f.render_widget(header_block, area);

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(28),
            Constraint::Min(0),
        ])
        .split(area);

    let very_simple_logo = vec![
        Line::from(""), 
        Line::from(Span::styled("  L U N I T O O L", Style::default().fg(logo_color).add_modifier(Modifier::BOLD))),
    ];

    let logo_paragraph = Paragraph::new(very_simple_logo)
        .alignment(Alignment::Left)
        .block(Block::default().padding(ratatui::widgets::Padding::new(2,0,0,0)));

    f.render_widget(logo_paragraph, header_chunks[0]);

    let lang_title_line1_full_str = get_text("LANG_TITLE_LINE1");
    let base_title_str = lang_title_line1_full_str.split(" - ").next().unwrap_or("").to_string();

    let subtitle_key = match state.current_screen { 
        Screen::LanguageSelect => "LANG_LANGUAGE_SELECT",
        Screen::KeyboardSelect => "LANG_KEYBOARD_SELECT",
        Screen::MainMenu => "LANG_MAIN_MENU",
        Screen::SystemInstallation => "INSTALL_HEADER_LINE2",
        _ => "LANG_SUBTITLE", 
    };
    let subtitle_str = get_text(subtitle_key);

    let title_line_spans = vec![
        Span::styled(base_title_str, Style::default().fg(main_title_color).add_modifier(Modifier::BOLD)),
        Span::styled(" ► ", Style::default().fg(breadcrumb_separator_color)),
        Span::styled(subtitle_str, Style::default().fg(breadcrumb_text_color).add_modifier(Modifier::BOLD)),
    ];
    
    let title_paragraph = Paragraph::new(Text::from(vec![Line::from("\n"), Line::from(title_line_spans)]))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(title_paragraph, header_chunks[1]);
}

fn draw_footer(f: &mut Frame, state: &UiState, area: Rect, theme: &Theme) {
    let footer_bg_color = theme.background_secondary;
    
    let key_label_color = theme.accent_primary; 
    let key_description_color = theme.text_primary; 

    let footer_block = Block::default()
        .borders(Borders::NONE)
        .bg(footer_bg_color);
    f.render_widget(footer_block, area);

    let mut all_hints = vec![
        Span::styled("Alt+T", Style::default().fg(key_label_color)),
        Span::styled(format!(": {} | ", get_text("LANG_TOGGLE_THEME_SHORT")), Style::default().fg(key_description_color)),
    ];

    let mut hints = Vec::new();

    match state.current_screen {
        Screen::SystemInstallation => {
            // Add Alt+L hint first for this screen
            hints.push(Span::styled("Alt+L", Style::default().fg(key_label_color)));
            hints.push(Span::styled(format!(": {} | ", get_text("LANG_TOGGLE_LOG_SHORT")), Style::default().fg(key_description_color)));
            // Then add the rest
            hints.push(Span::styled("Enter", Style::default().fg(key_label_color)));
            hints.push(Span::styled(format!(": {} | ", get_text("LANG_NEXT_STEP_SHORT")), Style::default().fg(key_description_color)));
            hints.push(Span::styled("Esc", Style::default().fg(key_label_color)));
            hints.push(Span::styled(format!(": {} | ", get_text("LANG_CANCEL_SHORT")), Style::default().fg(key_description_color)));
            hints.push(Span::styled("↑/↓", Style::default().fg(key_label_color)));
            hints.push(Span::styled(format!(": {} ", get_text("LANG_NAVIGATE_SHORT")), Style::default().fg(key_description_color)));
            hints.push(Span::styled(format!("| Backspace: {}", get_text("LANG_BACK_SHORT")), Style::default().fg(key_description_color)));
        }
        Screen::LanguageSelect | Screen::KeyboardSelect | Screen::MainMenu => {
            hints.push(Span::styled("↑/↓", Style::default().fg(key_label_color)));
            hints.push(Span::styled(format!(": {} | ", get_text("LANG_NAVIGATE_SHORT")), Style::default().fg(key_description_color)));
            hints.push(Span::styled("Enter", Style::default().fg(key_label_color)));
            hints.push(Span::styled(format!(": {} | ", get_text("LANG_SELECT_SHORT")), Style::default().fg(key_description_color)));
            hints.push(Span::styled("Backspace", Style::default().fg(key_label_color)));
            hints.push(Span::styled(format!(": {} | ", get_text("LANG_BACK_SHORT")), Style::default().fg(key_description_color)));
            hints.push(Span::styled("Esc", Style::default().fg(key_label_color)));
            hints.push(Span::styled(format!(": {}", get_text("LANG_EXIT_SHORT")), Style::default().fg(key_description_color)));
        }
        Screen::ConfirmExit | Screen::Message => {
            if let Some(DialogType::YesNo {..}) = state.active_dialog {
                hints.push(Span::styled("←/→", Style::default().fg(key_label_color)));
                hints.push(Span::styled(format!(": {} | ", get_text("LANG_NAVIGATE_SHORT")), Style::default().fg(key_description_color)));
                hints.push(Span::styled("Enter", Style::default().fg(key_label_color)));
                hints.push(Span::styled(format!(": {} | ", get_text("LANG_CONFIRM_SHORT")), Style::default().fg(key_description_color)));
                hints.push(Span::styled("Esc", Style::default().fg(key_label_color)));
                hints.push(Span::styled(format!(": {}", get_text("LANG_CANCEL_SHORT")), Style::default().fg(key_description_color)));
            } else { 
                hints.push(Span::styled("Enter/Esc", Style::default().fg(key_label_color)));
                hints.push(Span::styled(format!(": {}", get_text("LANG_CLOSE_SHORT")), Style::default().fg(key_description_color)));
            }
        }
    }

    all_hints.extend(hints);

    let legend_line = Line::from(all_hints).alignment(Alignment::Center);

    let legend = Paragraph::new(legend_line)
        .style(Style::default().fg(key_description_color))
        .block(Block::default()) 
        .alignment(Alignment::Center);

    let footer_centered_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    
            Constraint::Length(1), 
            Constraint::Min(0),    
        ])
        .split(area);

    f.render_widget(legend, footer_centered_chunks[1]);
}

/// Draw the main menu with card design
fn draw_main_menu(f: &mut Frame, state: &UiState, area: Rect, theme: &Theme) {
    let outer_block_area = area.inner(Margin { horizontal: 1, vertical: 0 });

    let description_text_color = theme.text_secondary;
    let border_color = theme.border_primary;
    let selected_border_color = theme.border_highlight;

    let block_bg_color = theme.background_secondary;
    let card_bg_color = theme.background_secondary;

    let main_block_title_style = Style::default().fg(theme.accent_secondary).add_modifier(Modifier::BOLD);
    let outer_block_border_style = Style::default().fg(border_color);

    let main_block_title = Span::styled(get_text("LANG_MAIN_MENU"), main_block_title_style);
    let outer_block = Block::default()
        .title(main_block_title)
        .borders(Borders::ALL)
        .border_style(outer_block_border_style)
        .border_type(BorderType::Rounded)
        .bg(block_bg_color); 
    
    f.render_widget(outer_block.clone(), outer_block_area);
    let inner_card_area = outer_block.inner(outer_block_area);
    
    let num_cards = state.menu_items.len();
    if num_cards == 0 {
        return;
    }

    let mut constraints = Vec::new();
    let spacer_percentage = if num_cards > 1 { 5 } else { 0 }; 
    let total_spacer_percentage = spacer_percentage * (num_cards.saturating_sub(1) as u16);
    let card_percentage = ((100u16.saturating_sub(total_spacer_percentage)) / (num_cards as u16).max(1)).max(1);

    for i in 0..num_cards {
        constraints.push(Constraint::Percentage(card_percentage));
        if i < num_cards - 1 {
            constraints.push(Constraint::Percentage(spacer_percentage));
        }
    }

    let card_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .margin(2) 
        .split(inner_card_area);
    
    for (idx, item) in state.menu_items.iter().enumerate() {
        if idx * 2 >= card_chunks.len() { break; } 
        let card_area = card_chunks[idx * 2]; 

        let is_selected = idx == state.selected_index;
        
        let current_card_border_style = if is_selected {
            Style::default().fg(selected_border_color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(border_color)
        };
        
        let current_title_text_color = if is_selected {
            theme.accent_secondary
        } else {
            theme.accent_primary
        };
            
        let card_border_block = Block::default()
            .title(Span::styled(&item.title, Style::default().fg(current_title_text_color).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(current_card_border_style)
            .border_type(BorderType::Rounded);
            
        f.render_widget(card_border_block.clone(), card_area); 
        
        let inner_area_for_bg = card_border_block.inner(card_area); 
        f.render_widget(Block::default().bg(card_bg_color), inner_area_for_bg);

        let description_area_margin = ratatui::layout::Margin { vertical: 1, horizontal: 1 }; 
        let description_area = inner_area_for_bg.inner(description_area_margin);
            
        if description_area.width > 0 && description_area.height > 0 {
            let final_description_color = if is_selected {
                if theme.name == "White Sur" {
                    Color::Black
                } else {
                    Color::White
                }
            } else {
                description_text_color
            };

            let description_paragraph = Paragraph::new(item.description.as_str())
                .style(Style::default().fg(final_description_color))
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Center); 
            f.render_widget(description_paragraph, description_area);
        }
    }
}

/// Draw the language selection screen
fn draw_language_select(f: &mut Frame, state: &UiState, area: Rect, theme: &Theme) {
    let outer_block_area = area.inner(Margin { horizontal: 1, vertical: 0 });

    let text_color = theme.text_primary;
    let border_color = theme.border_primary;
    
    let block_bg_color = theme.background_secondary;
    
    let selected_item_text_color = theme.list_item_selected_fg;
    let selected_item_bg_color = theme.list_item_selected_bg;
    let normal_item_text_color = text_color; 
    let normal_item_bg_color = theme.background_secondary;

    let block_title_style = Style::default().fg(theme.accent_secondary).add_modifier(Modifier::BOLD);
    let block_border_style = Style::default().fg(border_color);

    let block = Block::default()
        .title(Span::styled(get_text("LANG_LANGUAGE_SELECT"), block_title_style))
        .borders(Borders::ALL)
        .border_style(block_border_style)
        .border_type(BorderType::Rounded)
        .bg(block_bg_color);
    f.render_widget(block.clone(), outer_block_area);

    let list_items: Vec<ListItem> = state.languages.iter().enumerate().map(|(idx, lang_str)| {
            let display_text = match lang_str.as_str() {
                "de" => "Deutsch (de_DE)",
                "en" => "English (en_US)",
                _ => lang_str.as_str(),
            };
        
        let text_span = Span::raw(format!("  {}  ", display_text));

        if idx == state.selected_index {
            ListItem::new(text_span.style(Style::default().fg(selected_item_text_color).bg(selected_item_bg_color).add_modifier(Modifier::BOLD)))
        } else {
            ListItem::new(text_span.style(Style::default().fg(normal_item_text_color).bg(normal_item_bg_color)))
        }
    }).collect();

    let list_block_padding = 2; 
    let list_content_area = block.inner(outer_block_area).inner(ratatui::layout::Margin { vertical: list_block_padding, horizontal: list_block_padding });
    let max_text_width = state.languages.iter().map(|lang_str| {
        let display_text = match lang_str.as_str() {
            "de" => "Deutsch (de_DE)",
            "en" => "English (en_US)",
            _ => lang_str.as_str(),
        };
        display_text.chars().count() + 4 
    }).max().unwrap_or(20) as u16;
    let list_render_width = max_text_width.min(list_content_area.width);
    let list_horizontal_padding = list_content_area.width.saturating_sub(list_render_width) / 2;
    let centered_list_content_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(list_horizontal_padding),
            Constraint::Length(list_render_width),
            Constraint::Min(0),
        ])
        .split(list_content_area)[1];
    let num_items = list_items.len() as u16;
    let list_height = num_items;
    let final_list_area = if centered_list_content_area.height > list_height {
        let v_padding = (centered_list_content_area.height - list_height) / 2;
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(v_padding),
                Constraint::Length(list_height),
                Constraint::Min(0), 
            ])
            .split(centered_list_content_area)[1]
    } else {
        centered_list_content_area
    };
    let list = List::new(list_items)
        .block(Block::default().bg(block_bg_color)) 
        .highlight_style(Style::default().add_modifier(Modifier::BOLD)); 
    f.render_widget(list, final_list_area);
}

fn draw_keyboard_select(f: &mut Frame, state: &UiState, area: Rect, theme: &Theme) {
    let outer_block_area = area.inner(Margin { horizontal: 1, vertical: 0 });

    let text_color = theme.text_primary;
    let border_color = theme.border_primary;

    let block_bg_color = theme.background_secondary;

    let selected_item_text_color = theme.list_item_selected_fg;
    let selected_item_bg_color = theme.list_item_selected_bg;
    let normal_item_text_color = text_color; 
    let normal_item_bg_color = theme.background_secondary;

    let block_title_style = Style::default().fg(theme.accent_secondary).add_modifier(Modifier::BOLD);
    let block_border_style = Style::default().fg(border_color);

    let block = Block::default()
        .title(Span::styled(get_text("LANG_KEYBOARD_SELECT"), block_title_style))
        .borders(Borders::ALL)
        .border_style(block_border_style)
        .border_type(BorderType::Rounded)
        .bg(block_bg_color);
    f.render_widget(block.clone(), outer_block_area);

    let list_items: Vec<ListItem> = state.keyboards.iter().enumerate().map(|(idx, kb_str)| {
            let display_text = match kb_str.as_str() {
                "de" => "Deutsch (de)",
                "us" => "US-English (us)",
                _ => kb_str.as_str(),
            };
        let text_span = Span::raw(format!("  {}  ", display_text));

        if idx == state.selected_index {
            ListItem::new(text_span.style(Style::default().fg(selected_item_text_color).bg(selected_item_bg_color).add_modifier(Modifier::BOLD)))
        } else {
            ListItem::new(text_span.style(Style::default().fg(normal_item_text_color).bg(normal_item_bg_color)))
        }
    }).collect();

    let list_block_padding = 2;
    let list_content_area = block.inner(outer_block_area).inner(ratatui::layout::Margin { vertical: list_block_padding, horizontal: list_block_padding });
    let max_text_width = state.keyboards.iter().map(|kb_str| {
        let display_text = match kb_str.as_str() {
            "de" => "Deutsch (de)",
            "us" => "US-English (us)",
            _ => kb_str.as_str(),
        };
        display_text.chars().count() + 4
    }).max().unwrap_or(20) as u16;
    let list_render_width = max_text_width.min(list_content_area.width);
    let list_horizontal_padding = list_content_area.width.saturating_sub(list_render_width) / 2;
    let centered_list_content_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(list_horizontal_padding),
            Constraint::Length(list_render_width),
            Constraint::Min(0),
        ])
        .split(list_content_area)[1];
    let num_items = list_items.len() as u16;
    let list_height = num_items;
    let final_list_area = if centered_list_content_area.height > list_height {
        let v_padding = (centered_list_content_area.height - list_height) / 2;
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(v_padding),
                Constraint::Length(list_height),
                Constraint::Min(0),
            ])
            .split(centered_list_content_area)[1]
    } else {
        centered_list_content_area
    };
    let list = List::new(list_items)
        .block(Block::default().bg(block_bg_color)) 
        .highlight_style(Style::default().add_modifier(Modifier::BOLD)); 
    f.render_widget(list, final_list_area);
}

/// Draw a message dialog
fn draw_message(f: &mut Frame, state: &UiState, area: Rect, theme: &Theme) {
    let dialog_bg_color = theme.background_secondary;
    let border_color = theme.border_highlight; 
    let text_color = theme.text_primary; 

    if let Some((title, message)) = &state.message {
        let dialog_area = centered_rect_exact(60, 10, area); 
        
        let dialog_block_with_bg = Block::default()
            .title(Span::styled(title.as_str(), Style::default().fg(theme.accent_primary).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .border_type(BorderType::Rounded)
            .bg(dialog_bg_color); 

        f.render_widget(dialog_block_with_bg.clone(), dialog_area); 

        let content_inner_area = dialog_block_with_bg.inner(dialog_area);
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1), 
                Constraint::Length(1), 
                Constraint::Length(1), 
            ])
            .margin(1) 
            .split(content_inner_area); 

        let msg_paragraph = Paragraph::new(message.as_str())
            .style(Style::default().fg(text_color))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        f.render_widget(msg_paragraph, vertical_chunks[0]);
    }
}

/// Draw the exit confirmation dialog
fn draw_confirm_exit(f: &mut Frame, _state: &UiState, area: Rect, theme: &Theme) { 
    let screen_bg_color = theme.background_secondary; 

    let screen_block = Block::default()
        .borders(Borders::NONE) 
        .bg(screen_bg_color);
    f.render_widget(screen_block, area);
}

/// Helper function to create a centered rect with exact dimensions
fn centered_rect_exact(width: u16, height: u16, r: Rect) -> Rect {
    let x = r.x + (r.width.saturating_sub(width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    
    Rect {
        x,
        y,
        width: width.min(r.width),
        height: height.min(r.height),
    }
}

/// Formats byte sizes into human-readable strings.
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes == 0 {
        "0 B".to_string();
    }

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Builds a flattened list of display items representing disks, partitions,
/// LUKS containers, LVM structures, and filesystems for UI rendering.
pub fn build_disk_display_list(disk_info: &SystemDiskInfo) -> Vec<DisplayListItem> {
    let mut items = Vec::new();
    let indent_char = "  ";
    let luks_prefix = "🔒";
    let lvm_prefix = "📦";
    let disk_prefix = "💾";
    let part_prefix = "📄";
    let fs_prefix = "🗛";

    for disk in &disk_info.disks {
        let disk_id = disk.path.clone();
        let display_text = format!(
            "{} {} ({}, {})",
            disk_prefix,
            disk.path,
            disk.model.as_deref().unwrap_or("N/A"),
            format_size(disk.size_bytes)
        );
        items.push(DisplayListItem {
            id_path: disk_id,
            display_text,
            indent_level: 0,
            item_type: DisplayItemType::Disk,
            selectable: false, 
            size_bytes: Some(disk.size_bytes),
        });

        for partition in &disk.partitions {
            let part_id_base = partition.path.clone();
            let mut part_info_tags: Vec<String> = Vec::new();
            if let Some(fs_type) = &partition.fs_type {
                part_info_tags.push(fs_type.clone());
            }
            if let Some(label) = &partition.fs_label.as_ref().or(partition.part_label.as_ref()) {
                part_info_tags.push(format!("'{}'", label));
            }
            if let Some(mount_point) = &partition.mount_point {
                part_info_tags.push(format!("at '{}'", mount_point));
            }

            let part_display_text = format!(
                "{}└─ {} {} ({}, {})",
                indent_char.repeat(1),
                part_prefix,
                partition.path,
                format_size(partition.size_bytes),
                part_info_tags.join(", ")
            );
            items.push(DisplayListItem {
                id_path: part_id_base.clone(), 
                display_text: part_display_text,
                indent_level: 1,
                item_type: DisplayItemType::Partition,
                selectable: true,
                size_bytes: Some(partition.size_bytes),
            });

            if let Some(content) = &partition.content {
                match content {
                    PartitionContent::LuksContainer { uuid, mapped_name, mapped_content } => {
                        let luks_id = format!("{}/luks/{}", part_id_base, uuid);
                        let luks_display_text = format!(
                            "{}  └─ {} LUKS Container ({}){}",
                            indent_char.repeat(1),
                            luks_prefix,
                            uuid,
                            mapped_name.as_ref().map_or_else(|| " - Not active".to_string(), |mn| format!("  {}", mn))
                        );
                        items.push(DisplayListItem {
                            id_path: luks_id.clone(),
                            display_text: luks_display_text,
                            indent_level: 2,
                            item_type: DisplayItemType::LuksContainer,
                            selectable: mapped_name.is_none(),
                            size_bytes: Some(partition.size_bytes),
                        });

                        if let (Some(mn), Some(mc)) = (mapped_name, mapped_content) {
                            match mc.as_ref() {
                                MappedContent::LvmPhysicalVolume(pv_data) => {
                                    let pv_id = format!("{}/lvm_pv/{}", luks_id, pv_data.pv_uuid);
                                    let pv_text = format!(
                                        "{}    └─ {} LVM PV on {} (for VG: {})",
                                        indent_char.repeat(1),
                                        lvm_prefix,
                                        mn, 
                                        pv_data.vg_name.as_deref().unwrap_or("Unknown")
                                    );
                                    items.push(DisplayListItem {
                                        id_path: pv_id,
                                        display_text: pv_text,
                                        indent_level: 3,
                                        item_type: DisplayItemType::LvmPhysicalVolume,
                                        selectable: false,
                                        size_bytes: Some(pv_data.size_bytes),
                                    });
                                }
                                MappedContent::FileSystem { fs_type, fs_label, mount_point, .. } => {
                                    let fs_id = format!("{}/fs", luks_id);
                                    let mut fs_details_mc: Vec<String> = vec![fs_type.as_deref().unwrap_or("FS").to_string()];
                                    if let Some(fsl) = fs_label { fs_details_mc.push(format!("'{}'", fsl)); }
                                    if let Some(mp) = mount_point { fs_details_mc.push(format!("at '{}'", mp)); }
                                    let fs_text_mc = format!(
                                        "{}    └─ {} {} on {} ({})",
                                        indent_char.repeat(1),
                                        fs_prefix,
                                        fs_type.as_deref().unwrap_or("Filesystem"),
                                        mn,
                                        fs_details_mc.join(", ")
                                    );
                                    items.push(DisplayListItem {
                                        id_path: fs_id,
                                        display_text: fs_text_mc,
                                        indent_level: 3,
                                        item_type: DisplayItemType::FileSystemItem, 
                                        selectable: true,
                                        size_bytes: Some(partition.size_bytes),
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                    PartitionContent::LvmPhysicalVolume { pv_uuid, vg_name } => {
                        let lvm_pv_id = format!("{}/direct_lvm_pv/{}", part_id_base, pv_uuid);
                        let lvm_pv_text = format!(
                            "{}  └─ {} LVM PV (for VG: {})",
                            indent_char.repeat(1),
                            lvm_prefix,
                            vg_name.as_deref().unwrap_or("Unknown")
                        );
                        items.push(DisplayListItem {
                            id_path: lvm_pv_id,
                            display_text: lvm_pv_text,
                            indent_level: 2,
                            item_type: DisplayItemType::LvmPhysicalVolume,
                            selectable: false,
                            size_bytes: Some(partition.size_bytes),
                        });
                    }
                    PartitionContent::FileSystem => { /* Main partition line shows this info */ }
                    PartitionContent::Swap => { /* Main partition line fs_type indicates this */ }
                    PartitionContent::Unknown => {
                        items.push(DisplayListItem {
                            id_path: format!("{}/unallocated", part_id_base),
                            display_text: format!("{}  └─ Unallocated or Unknown Space", indent_char.repeat(1)),
                            indent_level: 2,
                            item_type: DisplayItemType::UnallocatedSpace,
                            selectable: true,
                            size_bytes: Some(partition.size_bytes),
                        });
                    }
                    _ => {} // VeraCrypt etc.
                }
            }
        }
    }

    if !disk_info.lvm_volume_groups.is_empty() {
        items.push(DisplayListItem {
            id_path: "lvm_section_header".to_string(),
            display_text: format!("\n{} LVM Volume Groups:", lvm_prefix),
            indent_level: 0,
            item_type: DisplayItemType::Label,
            selectable: false,
            size_bytes: None,
        });
        for vg in &disk_info.lvm_volume_groups {
            let vg_id = format!("lvm_vg/{}", vg.name);
            let vg_text = format!(
                "{} ({}, {} free, PVs: {})",
                vg.name,
                format_size(vg.size_bytes),
                format_size(vg.free_bytes),
                vg.physical_volumes.join(", ")
            );
            items.push(DisplayListItem {
                id_path: vg_id.clone(),
                display_text: vg_text,
                indent_level: 1, 
                item_type: DisplayItemType::LvmVolumeGroup,
                selectable: false, 
                size_bytes: Some(vg.size_bytes),
            });

            for lv in &vg.logical_volumes {
                let mut lv_details: Vec<String> = vec![format_size(lv.size_bytes)];
                if let Some(fs_type) = &lv.fs_type { lv_details.push(fs_type.clone()); }
                if let Some(label) = &lv.fs_label { lv_details.push(format!("'{}'", label)); }
                if let Some(mount) = &lv.mount_point { lv_details.push(format!("at '{}'", mount)); }
                
                let lv_text = format!(
                    "{}└─ {} {} ({}){}",
                    indent_char.repeat(1),
                    fs_prefix,
                    lv.name, 
                    lv_details.join(", "),
                    if lv.mount_point.as_deref() == Some("/") { " (Current System Root)".to_string() } else { "".to_string() }
                );
                items.push(DisplayListItem {
                    id_path: lv.path.clone(),
                    display_text: lv_text,
                    indent_level: 2,
                    item_type: DisplayItemType::LvmLogicalVolume,
                    selectable: true,
                    size_bytes: Some(lv.size_bytes),
                });
            }
        }
    }
    items
}

pub fn draw_installation_screen(
    f: &mut Frame,
    state: &mut UiState,
    area: Rect,
    theme: &Theme,
) {
    let additional_horizontal_margin: u16 = 1;

    let text_color = theme.text_primary;
    let base_block_bg_color = theme.background_secondary;

    let total_tasks = state.installation_tasks.len();
    let current_task_idx = state.current_installation_task_index;

    let calculated_progress_percent = if total_tasks == 0 {
        0
    } else if state.installation_step == Some(InstallationStep::Welcome) && current_task_idx == 0 {
        0 
    } else {
        let percentage = (current_task_idx as f32 / total_tasks as f32) * 100.0;
        percentage.round() as u16
    };
    let progress_percent = calculated_progress_percent.min(100); 

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),      
            Constraint::Length(6),   
            Constraint::Length(3),   
            Constraint::Length(1),   
        ].as_ref())
        .split(area);

    let main_content_and_task_list_area = chunks[0];
    let task_description_area_original = chunks[1];
    let gauge_area_original = chunks[2]; 
    let status_line_area_original = chunks[3]; 

    let adjusted_task_description_area = task_description_area_original.inner(
        Margin { horizontal: additional_horizontal_margin, vertical: 0 }
    );

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(75), 
            Constraint::Percentage(25),
        ])
        .split(main_content_and_task_list_area);

    let main_content_area_for_drawing_base = content_chunks[0];
    let task_list_base_chunk = content_chunks[1];

    let main_content_area_for_drawing = main_content_area_for_drawing_base.inner(
        Margin { horizontal: additional_horizontal_margin, vertical: 0 }
    );

    let task_list_area_for_drawing = Rect {
        x: task_list_base_chunk.x,
        y: task_list_base_chunk.y,
        width: (task_description_area_original.right() - additional_horizontal_margin).saturating_sub(task_list_base_chunk.x),
        height: task_list_base_chunk.height,
    };

    let gauge_fill_color = Color::Green;
    
    let progress_label_text = format!("{}%", progress_percent);
    let gauge_text_color = if theme.name == "White Sur" {
        if progress_percent >= 50 {
            theme.gauge_label_on_filled.unwrap_or(Color::White)
        } else {
            theme.gauge_label_on_empty.unwrap_or(Color::Black)
        }
    } else {
        Color::White
    };
    let progress_label = Span::styled(
        progress_label_text,
        Style::default().fg(gauge_text_color).add_modifier(Modifier::BOLD),
    );

    let gauge_border_color = theme.border_primary;
    let gauge_bg_color = base_block_bg_color;

    let gauge_widget = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(gauge_border_color))
                .border_type(BorderType::Rounded)
                .bg(gauge_bg_color) 
        )
        .gauge_style(Style::default().fg(gauge_fill_color).bg(gauge_bg_color)) 
        .percent(progress_percent)
        .label(progress_label); 

    let horizontal_narrowing: u16 = 2;
    let gauge_x = adjusted_task_description_area.x + horizontal_narrowing;
    let gauge_width = adjusted_task_description_area.width.saturating_sub(2 * horizontal_narrowing);

    let actual_gauge_render_area = Rect {
        x: gauge_x,
        y: gauge_area_original.y, 
        width: gauge_width,
        height: gauge_area_original.height, 
    };
    
    f.render_widget(gauge_widget, actual_gauge_render_area);

    if state.show_log_panel {
        let log_panel_bg = theme.log_panel_bg;
        let log_panel_text = theme.log_panel_text;
        let log_panel_border = theme.border_primary;

        let log_border_block = Block::default()
            .title(Span::styled(get_text("INSTALL_LOG_TITLE"), Style::default().fg(theme.accent_secondary)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(log_panel_border));

        f.render_widget(log_border_block.clone(), main_content_area_for_drawing);
        let inner_area_for_bg = log_border_block.inner(main_content_area_for_drawing);
        f.render_widget(Block::default().bg(log_panel_bg), inner_area_for_bg);

        let log_text_content = state.log_buffer.iter().map(|line| Line::from(line.as_str())).collect::<Vec<Line>>();
        let log_text_paragraph = Paragraph::new(log_text_content)
            .style(Style::default().fg(log_panel_text))
            .wrap(Wrap { trim: true })
            .block(Block::default()); 
        f.render_widget(log_text_paragraph, inner_area_for_bg.inner(Margin { vertical: 0, horizontal: 0 }));
    } else {
        draw_main_installation_content(f, state, main_content_area_for_drawing, theme);
    }

    draw_task_list(
        f, 
        state, 
        task_list_area_for_drawing, 
        theme,
    );
    
    draw_task_description(f, state, adjusted_task_description_area, theme);

    let adjusted_status_line_area = status_line_area_original.inner(
        Margin { horizontal: additional_horizontal_margin, vertical: 0 }
    );

    let current_step_label_text = get_text("INSTALL_LABEL_CURRENT_STEP");
    let command_status_label_text = get_text("INSTALL_LABEL_COMMAND_STATUS");
    
    let current_task_name = state.current_installation_task_name();
    let command_text = "<idle>"; 

    let status_label_color = theme.accent_primary; 
    let status_value_color = text_color;  
    let status_line_bg_color = base_block_bg_color;

    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), 
            Constraint::Percentage(40), 
        ])
        .split(adjusted_status_line_area); 

    let command_area = status_chunks[0];
    let step_area = status_chunks[1];

    let command_spans = Line::from(vec![
        Span::raw("  "), 
        Span::styled(command_status_label_text, Style::default().fg(status_label_color).add_modifier(Modifier::BOLD)),
        Span::raw(format!(" {}", command_text)).fg(status_value_color),
    ]).alignment(Alignment::Left);
    f.render_widget(Paragraph::new(command_spans).bg(status_line_bg_color), command_area);

    let step_spans = Line::from(vec![
        Span::styled(current_step_label_text, Style::default().fg(status_label_color).add_modifier(Modifier::BOLD)),
        Span::raw(format!(" {} ", current_task_name)).fg(status_value_color), 
        Span::raw("  "), 
    ]).alignment(Alignment::Right);
    f.render_widget(Paragraph::new(step_spans).bg(status_line_bg_color), step_area);
}

fn draw_main_installation_content(f: &mut Frame, state: &mut UiState, area: Rect, theme: &Theme) {
    let text_color = theme.text_primary;
    let border_color = theme.border_primary;
    let bg_color = theme.background_secondary;

    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .border_type(BorderType::Rounded)
        .bg(bg_color);
    
    let inner_area = main_block.inner(area);
    f.render_widget(main_block, area);

    let current_step_details = match state.installation_step.unwrap_or(InstallationStep::Welcome) {
        InstallationStep::Welcome => vec![
            Line::from(Span::styled(get_text("INSTALL_WELCOME_MESSAGE"), Style::default().fg(text_color))),
        ],
        InstallationStep::DiskSetup => {
            let mut lines = vec![];
            if state.is_loading_disks {
                lines.push(Line::from(Span::styled(get_text("INFO_LOADING_DISKS"), Style::default().fg(text_color))));
            } else if state.system_disk_info.is_none() || state.system_disk_info.as_ref().map_or(true, |sdi| sdi.disks.is_empty()) {
                lines.push(Line::from(Span::styled(get_text("INFO_NO_DISKS_FOUND"), Style::default().fg(text_color))));
            } else {
                lines.push(Line::from(Span::styled(get_text("PROMPT_SELECT_DISK"), Style::default().fg(text_color))));
            }
            lines
        }
        InstallationStep::UserSetup => {
            let prompt = format!("{}: ", get_text("PROMPT_HOSTNAME"));
            let input_text = state.input_buffer.as_str();
            vec![Line::from(Span::styled(format!("{}{}", prompt, input_text), Style::default().fg(text_color)))] 
        }
        _ => vec![Line::from(Span::styled(get_text("INFO_PENDING_IMPLEMENTATION"), Style::default().fg(text_color)))],
    };
    
    let content_paragraph = Paragraph::new(current_step_details)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(text_color));
    
    f.render_widget(content_paragraph, inner_area.inner(Margin { vertical: 1, horizontal: 1 }));
}

fn draw_task_description(f: &mut Frame, state: &mut UiState, area: Rect, theme: &Theme) {
    let text_color = theme.text_secondary;
    let border_color = theme.border_primary;
    let bg_color = theme.background_secondary;
    
    let scrollbar_color = theme.scrollbar_bg; 
    let scrollbar_thumb_color = theme.scrollbar_thumb;

    let description_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .border_type(BorderType::Rounded)
        .bg(bg_color);

    f.render_widget(description_block.clone(), area);
    
    let content_drawing_area = description_block.inner(area).inner(Margin { vertical: 1, horizontal: 1 });

    let description_text_key = match state.installation_step.unwrap_or(InstallationStep::Welcome) {
        InstallationStep::Welcome => "INSTALL_WELCOME_DESC",
        InstallationStep::DiskSetup => "INSTALL_DISK_SETUP_DESC",
        InstallationStep::UserSetup => "INSTALL_USER_SETUP_DESC",
        _ => "INFO_PENDING_IMPLEMENTATION",
    };
    let description_text_content = get_text(description_text_key);

    let lines: Vec<&str> = description_text_content.lines().collect();
    state.task_description_total_lines = lines.len();

    let visible_height = content_drawing_area.height as usize;
    let scrollbar_needed = state.task_description_total_lines > visible_height;

    let text_widget = Paragraph::new(description_text_content.as_str())
        .style(Style::default().fg(text_color))
        .wrap(Wrap { trim: false }) 
        .scroll((state.task_description_scroll_offset as u16, 0))
        .alignment(Alignment::Left);

    if scrollbar_needed {
        let layout_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),    
                Constraint::Length(1), 
            ])
            .split(content_drawing_area);
        
        let text_render_area = layout_chunks[0];
        let scrollbar_render_area = layout_chunks[1];

        f.render_widget(text_widget, text_render_area);

        let current_content_length = state.task_description_total_lines.saturating_sub(visible_height).max(0);
        state.task_description_scrollbar_state = state.task_description_scrollbar_state
            .content_length(current_content_length)
            .position(state.task_description_scroll_offset);
        
        if state.task_description_scroll_offset > current_content_length {
            state.task_description_scroll_offset = current_content_length;
        }

        f.render_stateful_widget(
            Scrollbar::default()
                .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(scrollbar_color).bg(bg_color))
                .thumb_style(Style::default().fg(scrollbar_thumb_color))
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            scrollbar_render_area,
            &mut state.task_description_scrollbar_state,
        );
    } else {
        f.render_widget(text_widget, content_drawing_area);
        state.task_description_scroll_offset = 0;
        state.task_description_scrollbar_state = state.task_description_scrollbar_state.content_length(0).position(0);
    }
}

fn draw_task_list(
    f: &mut Frame, 
    state: &mut UiState, 
    area: Rect, 
    theme: &Theme,
) {
    let text_color = theme.text_primary;
    let border_color = theme.border_primary;
    let bg_color = theme.background_secondary;
    
    let selected_text_color = theme.list_item_selected_fg;
    let selected_bg_color = theme.list_item_selected_bg;
    
    let active_status_color = theme.status_active.unwrap_or(theme.accent_primary); 
    let completed_status_color = theme.status_completed.unwrap_or(Color::Green);
    let failed_status_color = theme.status_failed.unwrap_or(Color::Red);
    let pending_status_color = theme.status_pending.unwrap_or(text_color); 

    let task_list_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .border_type(BorderType::Rounded)
        .bg(bg_color);

    let list_area = task_list_block.inner(area);
    f.render_widget(task_list_block, area);

    if state.installation_tasks.is_empty() {
        let no_tasks_msg = Paragraph::new(Text::styled(get_text("INFO_NO_TASKS"), Style::default().fg(text_color)))
            .alignment(Alignment::Center);
        f.render_widget(no_tasks_msg, list_area);
        return;
    }

    let task_items: Vec<ListItem> = state.installation_tasks.iter().enumerate().map(|(idx, task)| {
        let status_indicator_span = match task.status {
            InstallationTaskStatus::Active => Span::styled("► ", Style::default().fg(active_status_color)), 
            InstallationTaskStatus::Completed => Span::styled("● ", Style::default().fg(completed_status_color)),
            InstallationTaskStatus::Pending => Span::styled("○ ", Style::default().fg(pending_status_color)),
            InstallationTaskStatus::Failed => Span::styled("✗ ", Style::default().fg(failed_status_color)),
        };
        let title_span = Span::raw(task.title.clone()); 
        
        let item_style = if state.installation_task_list_state.selected() == Some(idx) {
            Style::default().fg(selected_text_color).bg(selected_bg_color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(text_color).bg(theme.background_secondary)
        };

        ListItem::new(Line::from(vec![status_indicator_span, title_span])).style(item_style)
    }).collect();

    let mut list_state_clone = state.installation_task_list_state.clone(); 

    let tasks_list_widget = List::new(task_items)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    
    f.render_stateful_widget(tasks_list_widget, list_area, &mut list_state_clone);
}

fn draw_active_dialog_popup(f: &mut Frame, state: &mut UiState, theme: &Theme) {
    if let Some(dialog_type) = state.active_dialog.clone() { 
        match dialog_type {
            DialogType::YesNo { title_key, message_key } => {
                let title_text = get_text(&title_key);
                let message_text = get_text(&message_key);

                let dialog_bg_color = theme.dialog_bg; 
                let dialog_text_color = theme.dialog_fg;
                let current_dialog_border_color = if title_key == "LANG_CONFIRM_TITLE" {
                    theme.border_highlight 
                } else {
                    theme.dialog_border 
                };

                let dialog_width: u16 = 60;
                let dialog_height: u16 = 8; 
                
                let popup_area = centered_rect_exact(
                    dialog_width,
                    dialog_height,
                    f.area(),
                );

                f.render_widget(Clear, popup_area);

                let block = Block::default()
                    .title(Span::styled(title_text, Style::default().fg(theme.dialog_title).add_modifier(Modifier::BOLD)))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(dialog_bg_color).fg(current_dialog_border_color));

                f.render_widget(block.clone(), popup_area);
                let inner_popup_area = block.inner(popup_area);

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1) 
                    .constraints([
                        Constraint::Min(1),    
                        Constraint::Length(1), 
                        Constraint::Length(1),    
                    ].as_ref())
                    .split(inner_popup_area);

                let message_paragraph = Paragraph::new(message_text)
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center)
                    .fg(dialog_text_color);
                f.render_widget(message_paragraph, chunks[0]);

                let button_area = chunks[2];
                let button_fg = theme.button_fg;
                let button_bg = theme.button_bg;

                let yes_text_content = format!("< {} >", get_text("DIALOG_YES"));
                let no_text_content = format!("< {} >", get_text("DIALOG_NO"));

                // Textfarbe für nicht-ausgewählte Buttons bestimmen
                let inactive_button_text_color = if theme.name == "White Sur" {
                    Color::Black
                } else {
                    Color::White
                };

                let yes_span = if state.dialog_selected_option == 0 {
                    Span::styled(yes_text_content, Style::default().fg(button_fg).bg(button_bg).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled(yes_text_content, Style::default().fg(inactive_button_text_color).bg(dialog_bg_color))
                };
                let no_span = if state.dialog_selected_option == 1 {
                    Span::styled(no_text_content, Style::default().fg(button_fg).bg(button_bg).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled(no_text_content, Style::default().fg(inactive_button_text_color).bg(dialog_bg_color))
                };

                // Buttonbereich horizontal aufteilen
                let button_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ].as_ref())
                    .split(button_area);

                f.render_widget(Paragraph::new(yes_span).alignment(Alignment::Center), button_chunks[0]);
                f.render_widget(Paragraph::new(no_span).alignment(Alignment::Center), button_chunks[1]);
            }
            DialogType::ThemeSelector => {
                draw_theme_selector_dialog(f, state, theme);
            }
        }
    }
}

fn draw_theme_selector_dialog(f: &mut Frame, state: &mut UiState, theme: &Theme) {
    let dialog_title_text = get_text("DIALOG_THEME_SELECTOR_TITLE");

    let dialog_bg_color = theme.dialog_bg;
    let dialog_text_color = theme.dialog_fg;
    let dialog_border_color = theme.border_highlight;
    let dialog_title_color = theme.dialog_title;
    let selected_option_bg = theme.dialog_selected_option_bg;
    let selected_option_text = theme.dialog_selected_option_text;

    let num_themes = state.themes.len() as u16;
    let dialog_content_height = num_themes; 
    let dialog_height = (dialog_content_height + 2 + 2).min(f.area().height.saturating_sub(4)); 
    let dialog_width: u16 = state.themes.iter()
        .map(|t| t.name.chars().count() as u16 + 4) 
        .max()
        .unwrap_or(30) 
        .max(30) 
        .min(f.area().width.saturating_sub(4)); 

    let popup_area = centered_rect_exact(dialog_width, dialog_height, f.area());

    f.render_widget(Clear, popup_area); 

    let block = Block::default()
        .title(Span::styled(dialog_title_text, Style::default().fg(dialog_title_color).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(dialog_bg_color).fg(dialog_border_color));

    f.render_widget(block.clone(), popup_area);
    let list_content_area = block.inner(popup_area).inner(Margin { vertical: 1, horizontal: 0 }); 

    if state.themes.is_empty() {
        let no_themes_text = get_text("DIALOG_THEME_SELECTOR_NO_THEMES");
        let paragraph = Paragraph::new(no_themes_text)
            .style(Style::default().fg(dialog_text_color))
            .alignment(Alignment::Center);
        f.render_widget(paragraph, list_content_area);
        return;
    }

    let items: Vec<ListItem> = state.themes.iter().enumerate().map(|(idx, theme_item)| {
        let item_text_raw = theme_item.name.clone();
        let available_width = list_content_area.width; 
        let text_len = item_text_raw.chars().count() as u16;
        let padding_needed = available_width.saturating_sub(text_len);
        let left_padding = padding_needed / 2;
        let right_padding = padding_needed.saturating_sub(left_padding);
        
        let item_text = format!("{}{}{}", " ".repeat(left_padding as usize), item_text_raw, " ".repeat(right_padding as usize));

        if idx == state.dialog_selected_option {
            ListItem::new(Span::styled(item_text, Style::default().fg(selected_option_text).bg(selected_option_bg).add_modifier(Modifier::BOLD)))
        } else {
            ListItem::new(Span::styled(item_text, Style::default().fg(dialog_text_color).bg(dialog_bg_color)))
        }
    }).collect();
    
    let mut list_state_for_render = ListState::default();
    list_state_for_render.select(Some(state.dialog_selected_option));

    let list_widget = List::new(items)
        .block(Block::default()) 
        .highlight_style(Style::default().add_modifier(Modifier::BOLD)); 

    f.render_stateful_widget(list_widget, list_content_area, &mut list_state_for_render);
}