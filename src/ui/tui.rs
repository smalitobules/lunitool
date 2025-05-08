use anyhow::{Context, Result};
use crossterm::{
    event::{EnableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, BorderType, List, ListItem, Paragraph, Clear},
    Frame, Terminal,
};

use crate::{
    lang::get_text,
    ui::widgets::{MenuType, Screen},
};

/// UI state for the application
#[derive(Debug)]
pub struct UiState {
    /// Current screen
    pub current_screen: Screen,
    /// Main menu items
    pub menu_items: Vec<crate::ui::widgets::MenuItem>,
    /// Selected menu index
    pub selected_index: usize,
    /// Available languages
    pub languages: Vec<String>,
    /// Available keyboard layouts
    pub keyboards: Vec<String>,
    /// Current message (if any)
    pub message: Option<(String, String)>, // (title, content)
    /// Previous screen (for going back)
    pub previous_screen: Screen,
}

impl UiState {
    /// Create a new UI state
    pub fn new(menu_items: Vec<crate::ui::widgets::MenuItem>) -> Self {
        Self {
            current_screen: Screen::LanguageSelect,
            menu_items,
            selected_index: 0,
            languages: vec!["de".to_string(), "en".to_string()],
            keyboards: vec!["de".to_string(), "us".to_string()],
            message: None,
            previous_screen: Screen::LanguageSelect,
        }
    }

    /// Get selected menu item
    pub fn selected_menu_item(&self) -> Option<&crate::ui::widgets::MenuItem> {
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
}

/// Setup the terminal
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    // Setup terminal
    terminal::enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).context("Failed to create terminal")?;
    Ok(terminal)
}

/// Draw the UI based on current state
pub fn draw_ui<B: Backend>(f: &mut Frame<B>, state: &UiState) {
    // Render a full-screen clear to eliminate potential artifacts
    f.render_widget(Clear, f.size());
    
    // Create the base layout with appropriate padding
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)  // Add margin to the whole layout
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(1),     // Content
            Constraint::Length(3),  // Status bar
        ])
        .split(f.size());

    // Draw title with proper styling
    let title = Paragraph::new(Spans::from(vec![
        Span::styled(get_text("LANG_TITLE"), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" - "),
        Span::styled(get_text("LANG_SUBTITLE"), Style::default().fg(Color::White)),
    ]))
    .alignment(Alignment::Center)  // Center the title
    .block(Block::default().borders(Borders::BOTTOM));
    
    f.render_widget(title, main_layout[0]);

    // Draw content based on current screen
    match state.current_screen {
        Screen::MainMenu => draw_main_menu(f, state, main_layout[1]),
        Screen::LanguageSelect => draw_language_select(f, state, main_layout[1]),
        Screen::KeyboardSelect => draw_keyboard_select(f, state, main_layout[1]),
        Screen::Message => draw_message(f, state, main_layout[1]),
        Screen::ConfirmExit => draw_confirm_exit(f, state, main_layout[1]),
    }

    // Draw status bar
    let navigation_help = Paragraph::new(get_text("LANG_NAVIGATION"))
        .alignment(Alignment::Center)  // Center the navigation text
        .style(Style::default().fg(Color::Gray));
    f.render_widget(navigation_help, main_layout[2]);
}

/// Draw the main menu with card design
fn draw_main_menu<B: Backend>(f: &mut Frame<B>, state: &UiState, area: Rect) {
    // Calculate a centered area for the menu items - make it square-ish
    let square_size = area.height.min(area.width);
    let menu_height = (square_size as f32 * 0.8) as u16;
    let menu_width = (square_size as f32 * 0.9) as u16;
    
    let menu_area = centered_rect_exact(menu_width, menu_height, area);
    
    // Create an outer border for the menu area
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray));
    f.render_widget(outer_block, menu_area);
    
    // Create an inner area inside the outer border with padding
    let inner_area = Rect {
        x: menu_area.x + 2,
        y: menu_area.y + 2,
        width: menu_area.width - 4,
        height: menu_area.height - 4,
    };
    
    // Create a grid layout for the menu cards
    let menu_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)  // Add margin between cards
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(inner_area);
    
    // Render each menu card
    for (i, item) in state.menu_items.iter().enumerate() {
        if i < menu_layout.len() {
            // Create card area with fixed height to make it more square
            let card_height = inner_area.height.min(menu_layout[i].width); // Makes it square-ish
            
            let card_area = Rect {
                x: menu_layout[i].x,
                y: menu_layout[i].y,
                width: menu_layout[i].width,
                height: card_height,
            };
            
            // Select the appropriate style based on selection
            let border_style = if i == state.selected_index {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            let text_style = if i == state.selected_index {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            
            // Create a block with a rounded border
            let block = Block::default()
                .title(Span::styled(format!("  {}  ", item.title), Style::default().add_modifier(Modifier::BOLD)))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style);
            
            // Render the card block
            f.render_widget(block, card_area);
            
            // Calculate the description area inside the card
            let description_area = Rect {
                x: card_area.x + 2,
                y: card_area.y + 2,
                width: card_area.width - 4,
                height: card_area.height - 4,
            };
            
            // Render the description
            let description = Paragraph::new(item.description.clone())
                .alignment(Alignment::Center)
                .style(text_style)
                .wrap(tui::widgets::Wrap { trim: true });
            
            f.render_widget(description, description_area);
        }
    }
}

/// Draw the language selection screen
fn draw_language_select<B: Backend>(f: &mut Frame<B>, state: &UiState, area: Rect) {
    let items: Vec<ListItem> = state.languages
        .iter()
        .enumerate()
        .map(|(i, lang)| {
            let display = match lang.as_str() {
                "de" => "Deutsch (de_DE)",
                "en" => "English (en_US)",
                _ => lang,
            };
            
            ListItem::new(Spans::from(vec![
                Span::styled(display, Style::default().fg(if i == state.selected_index { Color::Green } else { Color::White })),
            ]))
        })
        .collect();

    // Create a centered area for the language selection
    let lang_area = centered_rect(60, 40, area);
    
    let menu = List::new(items)
        .block(Block::default()
            .title(get_text("LANG_LANGUAGE_SELECT"))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(menu, lang_area);
}

/// Draw the keyboard layout selection screen
fn draw_keyboard_select<B: Backend>(f: &mut Frame<B>, state: &UiState, area: Rect) {
    let items: Vec<ListItem> = state.keyboards
        .iter()
        .enumerate()
        .map(|(i, kb)| {
            let display = match kb.as_str() {
                "de" => "Deutsch (de)",
                "us" => "US-English (us)",
                _ => kb,
            };
            
            ListItem::new(Spans::from(vec![
                Span::styled(display, Style::default().fg(if i == state.selected_index { Color::Green } else { Color::White })),
            ]))
        })
        .collect();

    // Create a centered area for the keyboard selection
    let kb_area = centered_rect(60, 40, area);

    let menu = List::new(items)
        .block(Block::default()
            .title(get_text("LANG_KEYBOARD_SELECT"))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(menu, kb_area);
}

/// Draw a message dialog
fn draw_message<B: Backend>(f: &mut Frame<B>, state: &UiState, area: Rect) {
    if let Some((title, content)) = &state.message {
        // Create a centered box for the message
        let message_area = centered_rect(60, 40, area);
        
        // Render a clear background to make the dialog stand out
        f.render_widget(Clear, message_area);
        
        let message = Paragraph::new(content.as_str())
            .block(Block::default()
                .title(title.as_str())
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::White)))
            .alignment(Alignment::Center)
            .wrap(tui::widgets::Wrap { trim: true });
        
        f.render_widget(message, message_area);
    }
}

/// Draw the exit confirmation dialog
fn draw_confirm_exit<B: Backend>(f: &mut Frame<B>, state: &UiState, area: Rect) {
    // Create a centered box for the dialog with proper padding
    let dialog_area = centered_rect(50, 30, area);
    
    // Render a clear background to make the dialog stand out
    f.render_widget(Clear, dialog_area);
    
    // Create a block with rounded borders
    let block = Block::default()
        .title("Confirm")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White));
    
    f.render_widget(block, dialog_area);
    
    // Create layout for the dialog content with padding
    let inner_area = Rect {
        x: dialog_area.x + 2,
        y: dialog_area.y + 2,
        width: dialog_area.width - 4,
        height: dialog_area.height - 4,
    };
    
    let dialog_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1), // Message
            Constraint::Length(1), // Buttons
            Constraint::Length(1), // Extra padding
        ])
        .split(inner_area);
    
    // Render the message
    let message = Paragraph::new(get_text("LANG_EXIT_CONFIRM"))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));
    f.render_widget(message, dialog_layout[0]);
    
    // Determine the correct Yes/No text based on the current language
    let yes_text = get_text("LANG_YES");
    let no_text = get_text("LANG_NO");
    
    // Determine yes/no key hints based on language
    let yes_key = if yes_text == "Ja" { "j" } else { "y" };
    let no_key = if no_text == "Nein" { "n" } else { "n" };
    
    // Highlight the appropriate button based on selection
    let yes_style = if state.selected_index == 0 {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    
    let no_style = if state.selected_index == 1 {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    
    // Create the buttons text
    let buttons = Paragraph::new(Spans::from(vec![
        Span::styled(format!("{} ({})", yes_text, yes_key), yes_style),
        Span::raw("    "),
        Span::styled(format!("{} ({})", no_text, no_key), no_style),
    ]))
    .alignment(Alignment::Center);
    
    f.render_widget(buttons, dialog_layout[1]);
}

/// Helper function to create a centered rect using percentages
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
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