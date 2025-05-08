use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::{time::{Duration, Instant}};
use tui::{
    backend::Backend,
    Terminal,
};

use crate::{
    config::Config,
    core::{system_info::collect_system_info, check_root},
    ui::{
        tui::{draw_ui, UiState},
        widgets::{MenuItem, MenuType, Screen},
    },
    lang::get_text,
};

/// Application state
pub struct App<B: Backend + std::io::Write> {
    /// Terminal to draw UI on
    terminal: Terminal<B>,
    /// Application configuration
    config: Config,
    /// Current UI state
    ui_state: UiState,
    /// Should the application exit
    should_quit: bool,
    /// Last key pressed timestamp
    last_tick: Instant,
    /// Tick rate for UI updates
    tick_rate: Duration,
}

impl<B: Backend + std::io::Write> App<B> {
    /// Create a new App
    pub fn new(config: Config, terminal: Terminal<B>) -> Self {
        // Collect system information at startup
        let system_info = collect_system_info();
        log::info!("System info collected: {:?}", system_info);
        
        // Check if running as root
        if !check_root() {
            log::warn!("Application not running as root. Some features may be limited.");
        }
        
        // Create main menu items
        let menu_items = vec![
            MenuItem {
                id: "install".to_string(),
                title: get_text("LANG_INSTALL"),
                description: get_text("LANG_INSTALL_DESC"),
                menu_type: MenuType::Card,
            },
            MenuItem {
                id: "backup".to_string(),
                title: get_text("LANG_BACKUP"),
                description: get_text("LANG_BACKUP_DESC"),
                menu_type: MenuType::Card,
            },
            MenuItem {
                id: "keys".to_string(),
                title: get_text("LANG_KEYS"),
                description: get_text("LANG_KEYS_DESC"),
                menu_type: MenuType::Card,
            },
        ];

        Self {
            terminal,
            config,
            ui_state: UiState::new(menu_items),
            should_quit: false,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(100),
        }
    }

    /// Run the application main loop
    pub fn run(&mut self) -> Result<()> {
        // Initial UI draw
        self.terminal.draw(|f| draw_ui(f, &self.ui_state))?;

        // Main event loop
        while !self.should_quit {
            // Wait for next event
            if event::poll(self.tick_rate.saturating_sub(self.last_tick.elapsed()))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key_event(key.code);
                }
            }

            // Handle tick for animations etc.
            if self.last_tick.elapsed() >= self.tick_rate {
                self.last_tick = Instant::now();
                self.terminal.draw(|f| draw_ui(f, &self.ui_state))?;
            }
        }

        Ok(())
    }

    /// Handle keyboard input
    fn handle_key_event(&mut self, key_code: KeyCode) {
        match self.ui_state.current_screen {
            Screen::MainMenu => {
                match key_code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.confirm_exit();
                    },
                    KeyCode::Enter => {
                        let selected = self.ui_state.selected_menu_item();
                        if let Some(menu_item) = selected {
                            match menu_item.id.as_str() {
                                "install" => {
                                    log::info!("Starting installation module...");
                                    // TODO: Implement installation module
                                    self.ui_state.show_message(&get_text("LANG_INSTALL"), &get_text("LANG_NOT_IMPLEMENTED"));
                                },
                                "backup" => {
                                    log::info!("Starting backup module...");
                                    // TODO: Implement backup module
                                    self.ui_state.show_message(&get_text("LANG_BACKUP"), &get_text("LANG_NOT_IMPLEMENTED"));
                                },
                                "keys" => {
                                    log::info!("Starting key management module...");
                                    // TODO: Implement key management module
                                    self.ui_state.show_message(&get_text("LANG_KEYS"), &get_text("LANG_NOT_IMPLEMENTED"));
                                },
                                _ => {
                                    log::warn!("Unknown menu item selected: {}", menu_item.id);
                                }
                            }
                        }
                    },
                    KeyCode::Down | KeyCode::Right => {
                        self.ui_state.next_menu_item();
                    },
                    KeyCode::Up | KeyCode::Left => {
                        self.ui_state.previous_menu_item();
                    },
                    KeyCode::Backspace => {
                        // Navigate back to keyboard selection
                        self.ui_state.selected_index = 0;
                        self.ui_state.current_screen = Screen::KeyboardSelect;
                    },
                    _ => {}
                }
            },
            Screen::LanguageSelect => {
                match key_code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.confirm_exit();
                    },
                    KeyCode::Enter => {
                        if let Some(lang) = self.ui_state.selected_language() {
                            let lang_clone = lang.clone();
                            self.config.current_lang = lang;
                            if let Err(e) = crate::core::load_language(&lang_clone) {
                                log::error!("Failed to load language: {}", e);
                                self.ui_state.show_error("Language Error", &format!("Failed to load language: {}", e));
                            } else {
                                log::info!("Language changed to: {}", lang_clone);
                                
                                // Update the menu items with the new language
                                let menu_items = vec![
                                    MenuItem {
                                        id: "install".to_string(),
                                        title: get_text("LANG_INSTALL"),
                                        description: get_text("LANG_INSTALL_DESC"),
                                        menu_type: MenuType::Card,
                                    },
                                    MenuItem {
                                        id: "backup".to_string(),
                                        title: get_text("LANG_BACKUP"),
                                        description: get_text("LANG_BACKUP_DESC"),
                                        menu_type: MenuType::Card,
                                    },
                                    MenuItem {
                                        id: "keys".to_string(),
                                        title: get_text("LANG_KEYS"),
                                        description: get_text("LANG_KEYS_DESC"),
                                        menu_type: MenuType::Card,
                                    },
                                ];
                                self.ui_state.menu_items = menu_items;
                                
                                // Passe die Tastaturlayouts basierend auf der Sprache an
                                if lang_clone == "en" {
                                    self.ui_state.keyboards = vec!["us".to_string(), "de".to_string()];
                                } else {
                                    self.ui_state.keyboards = vec!["de".to_string(), "us".to_string()];
                                }
                                
                                // Reset selected index for keyboard selection
                                self.ui_state.selected_index = 0;
                                self.ui_state.current_screen = Screen::KeyboardSelect;
                            }
                        }
                    },
                    KeyCode::Down => {
                        self.ui_state.next_language();
                    },
                    KeyCode::Up => {
                        self.ui_state.previous_language();
                    },
                    _ => {}
                }
            },
            Screen::KeyboardSelect => {
                match key_code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.confirm_exit();
                    },
                    KeyCode::Backspace => {
                        // Go back to language selection
                        self.ui_state.selected_index = 0;
                        self.ui_state.current_screen = Screen::LanguageSelect;
                    },
                    KeyCode::Enter => {
                        if let Some(kb) = self.ui_state.selected_keyboard() {
                            let kb_clone = kb.clone();
                            self.config.keyboard = kb;
                            if let Err(e) = crate::core::set_keyboard(&kb_clone) {
                                log::error!("Failed to set keyboard layout: {}", e);
                                self.ui_state.show_error("Keyboard Error", &format!("Failed to set keyboard layout: {}", e));
                            } else {
                                log::info!("Keyboard layout changed to: {}", kb_clone);
                                // Reset selected index for main menu
                                self.ui_state.selected_index = 0;
                                self.ui_state.current_screen = Screen::MainMenu;
                            }
                        }
                    },
                    KeyCode::Down => {
                        self.ui_state.next_keyboard();
                    },
                    KeyCode::Up => {
                        self.ui_state.previous_keyboard();
                    },
                    _ => {}
                }
            },
            Screen::Message => {
                match key_code {
                    KeyCode::Enter | KeyCode::Esc | KeyCode::Backspace => {
                        self.ui_state.clear_message();
                    },
                    _ => {}
                }
            },
            Screen::ConfirmExit => {
                match key_code {
                    // Y or J for "Yes" based on language
                    KeyCode::Char('y') | KeyCode::Char('j') => {
                        self.should_quit = true;
                    },
                    // N for "No" (works in both languages)
                    KeyCode::Char('n') | KeyCode::Esc => {
                        // Go back to previous screen
                        self.ui_state.current_screen = self.ui_state.previous_screen;
                    },
                    KeyCode::Right | KeyCode::Left => {
                        // Toggle selection between Yes and No
                        self.ui_state.selected_index = 1 - self.ui_state.selected_index;
                    },
                    KeyCode::Enter => {
                        if self.ui_state.selected_index == 0 {
                            // Selected "Yes"
                            self.should_quit = true;
                        } else {
                            // Selected "No"
                            self.ui_state.current_screen = self.ui_state.previous_screen;
                        }
                    },
                    KeyCode::Backspace => {
                        // Go back to previous screen (cancel)
                        self.ui_state.current_screen = self.ui_state.previous_screen;
                    },
                    _ => {}
                }
            },
        }
    }

    /// Show exit confirmation dialog
    fn confirm_exit(&mut self) {
        self.ui_state.previous_screen = self.ui_state.current_screen;
        self.ui_state.current_screen = Screen::ConfirmExit;
        self.ui_state.selected_index = 1; // Default to "No"
        self.terminal.draw(|f| draw_ui(f, &self.ui_state)).unwrap();
    }

    /// Restore terminal to original state
    pub fn restore_terminal(&mut self) -> Result<()> {
        self.terminal.clear()?;
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            self.terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        )?;
        Ok(())
    }
}