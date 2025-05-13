use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::{time::{Duration, Instant}};
use ratatui::{
    backend::Backend,
    Terminal,
};

use crate::{
    config::Config,
    core::{system_info::collect_system_info, check_root, disk_info::create_dummy_system_disk_info},
    ui::{
        tui::{draw_ui, UiState, build_disk_display_list},
        widgets::{MenuItem, MenuType, Screen, DialogType},
    },
    lang::get_text,
};

// Installation wizard related enums and structs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstallationStep {
    Welcome,
    DiskSetup,
    UserSetup,
    NetworkConfig,
    DesktopChoice,
    KernelChoice,
    SecureBootChoice,
    UpdateSettings,
    AdditionalPackages,
    Summary,
    Installing,
    Completed,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstallationTaskStatus {
    Pending,
    Active,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct InstallationTaskItem {
    pub id: String,
    pub title: String,
    pub step: InstallationStep,
    pub status: InstallationTaskStatus,
}

#[derive(Debug, Clone, Default)]
pub struct InstallationConfig {
    pub target_disk: Option<String>,
    pub hostname: Option<String>,
    pub username: Option<String>,
    // Passwords should not be stored in plain text here long-term,
    // but might be needed temporarily during collection before passing to scripts.
    // Consider secure handling. For now, as Option<String>.
    pub user_password: Option<String>,
    pub luks_password: Option<String>,
}

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
        let system_info = collect_system_info();
        log::info!("System info collected: {:?}", system_info);
        
        if !check_root() {
            log::warn!("Application not running as root. Some features may be limited.");
        }
        
        // Define menu items once, ensuring they are localized after language is set.
        // Language is set first, then menu items are created.
        if let Err(e) = crate::core::load_language(&config.current_lang) {
            log::error!("App::new: Failed to apply initial language from config '{}': {}", config.current_lang, e);
            // Optionally, fallback to a default language or handle error more gracefully
            // For now, we proceed, hoping a default bundle might have been loaded by lazy_static.
        }

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

        let mut ui_state = UiState::new(menu_items); // menu_items passed here

        // Initialize disk info with dummy data for now
        ui_state.system_disk_info = Some(create_dummy_system_disk_info());
        if let Some(info) = &ui_state.system_disk_info {
            if !info.disks.is_empty() {
                 // Try to select the first disk or partition for initial view
                ui_state.disk_setup_list_state.select(Some(0)); 
            }
        }

        // Adjust available keyboards based on initial language
        if config.current_lang == "en" {
            ui_state.keyboards = vec!["us".to_string(), "de".to_string()];
        } else {
            ui_state.keyboards = vec!["de".to_string(), "us".to_string()];
        }

        Self {
            terminal,
            config,
            ui_state,
            should_quit: false,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(100),
        }
    }

    /// Run the application main loop
    pub fn run(&mut self) -> Result<()> {
        self.terminal.draw(|f| draw_ui(f, &mut self.ui_state))?;

        while !self.should_quit {
            if event::poll(self.tick_rate.saturating_sub(self.last_tick.elapsed()))? {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == crossterm::event::KeyEventKind::Press => {
                        self.handle_key_event(key_event);
                    }
                    Event::FocusGained => { log::debug!("Focus Gained"); }
                    Event::FocusLost => { log::debug!("Focus Lost"); }
                    Event::Mouse(mouse_event) => { log::debug!("Mouse Event: {:?}", mouse_event); }
                    Event::Paste(pasted_text) => { log::debug!("Pasted text: {}", pasted_text); }
                    Event::Resize(width, height) => { 
                        log::info!("Terminal resized to {}x{}", width, height); 
                        // Force redraw on resize
                        self.terminal.draw(|f| draw_ui(f, &mut self.ui_state))?;
                    }
                    _ => {}
                }
            }

            // Handle tick for animations etc.
            if self.last_tick.elapsed() >= self.tick_rate {
                self.last_tick = Instant::now();
                self.terminal.draw(|f| draw_ui(f, &mut self.ui_state))?;
            }
        }

        Ok(())
    }

    /// Handle keyboard input
    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        let key_code = key.code;
        let modifiers = key.modifiers;

        // Alt + L for Log Panel & Alt + T for Theme Selection Dialog
        if modifiers == crossterm::event::KeyModifiers::ALT {
            match key_code {
                KeyCode::Char('l') | KeyCode::Char('L') => {
                    if self.ui_state.current_screen == Screen::SystemInstallation {
                        self.ui_state.show_log_panel = !self.ui_state.show_log_panel;
                        log::info!("Log panel toggled with Alt+L: {}", self.ui_state.show_log_panel);
                        return;
                    }
                }
                KeyCode::Char('t') | KeyCode::Char('T') => { // Alt+T for Theme Selection
                    if self.ui_state.active_dialog.is_none() { // Only open if no other dialog is active
                        log::info!("Opening Theme Selector dialog.");
                        self.ui_state.active_dialog = Some(DialogType::ThemeSelector);
                        self.ui_state.dialog_selected_option = self.ui_state.active_theme_index; // Pre-select current theme
                        return;
                    }
                }
                _ => {}
            }
        }

        // First, check if an active dialog needs to handle the input
        if self.ui_state.active_dialog.is_some() {
            // Clone to avoid borrowing issues with self.ui_state in match
            let dialog_type_clone = self.ui_state.active_dialog.clone();

            match dialog_type_clone {
                Some(DialogType::YesNo {..}) => {
                    match key_code {
                        KeyCode::Left | KeyCode::Char('h') => {
                            if self.ui_state.dialog_selected_option == 1 {
                                self.ui_state.dialog_selected_option = 0;
                            }
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            if self.ui_state.dialog_selected_option == 0 {
                                self.ui_state.dialog_selected_option = 1;
                            }
                        }
                        KeyCode::Char('y') | KeyCode::Char('j') => {
                            self.ui_state.dialog_selected_option = 0;
                        }
                        KeyCode::Char('n') => {
                            self.ui_state.dialog_selected_option = 1;
                        }
                        KeyCode::Enter => {
                            self.handle_dialog_confirm();
                        }
                        KeyCode::Esc | KeyCode::Backspace => {
                            self.handle_dialog_cancel();
                        }
                        _ => {}
                    }
                }
                Some(DialogType::ThemeSelector) => {
                    match key_code {
                        KeyCode::Up | KeyCode::Char('k') => {
                            if !self.ui_state.themes.is_empty() {
                                if self.ui_state.dialog_selected_option > 0 {
                                    self.ui_state.dialog_selected_option -= 1;
                                } else {
                                    self.ui_state.dialog_selected_option = self.ui_state.themes.len() - 1; // Wrap around
                                }
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if !self.ui_state.themes.is_empty() {
                                self.ui_state.dialog_selected_option = (self.ui_state.dialog_selected_option + 1) % self.ui_state.themes.len(); // Wrap around
                            }
                        }
                        KeyCode::Enter => {
                            self.handle_theme_dialog_confirm();
                        }
                        KeyCode::Esc | KeyCode::Backspace => {
                            self.handle_dialog_cancel(); // Closes the dialog without changes
                        }
                        _ => {}
                    }
                }
                None => { /* Should not happen, as active_dialog.is_some() is checked */ }
            }
            return; // Input handled by dialog, no further processing for this key event
        }

        // Global ESC to confirm exit, if no dialog is active and not already in confirm exit/message screen.
        if key_code == KeyCode::Esc && 
           self.ui_state.active_dialog.is_none() && // Ensure no other dialog is already active
           self.ui_state.current_screen != Screen::ConfirmExit && // Keep this condition for now
           self.ui_state.current_screen != Screen::Message {
            self.confirm_exit();
            return; 
        }

        // Global Backspace to go to previous logical step or screen
        if key_code == KeyCode::Backspace && 
           self.ui_state.current_screen != Screen::Message && 
           self.ui_state.current_screen != Screen::ConfirmExit {
            
            // Specific back navigation for certain screens BEFORE global previous_screen logic
            match self.ui_state.current_screen {
                Screen::SystemInstallation => {
                self.handle_installation_previous_step(); 
                return;
            }
                Screen::KeyboardSelect => {
                    self.ui_state.set_current_screen(Screen::LanguageSelect);
                    return;
                }
                Screen::MainMenu => {
                    self.ui_state.set_current_screen(Screen::KeyboardSelect);
                    return;
                }
                _ => {}
            }
            
            // Fallback to previous screen if not handled above and previous_screen is different
            if self.ui_state.previous_screen != self.ui_state.current_screen {
                 // Avoid going back from MainMenu to KeyboardSelect this way if Backspace on MainMenu should be Exit
                if self.ui_state.current_screen == Screen::LanguageSelect && self.ui_state.previous_screen == Screen::KeyboardSelect {
                    // This case can happen if someone navigates L->K->M, then ESC from M to K, then ESC from K to L.
                    // Backspace on LanguageSelect should probably lead to exit confirmation.
                    self.confirm_exit();
                    return;
                }
                 self.ui_state.set_current_screen(self.ui_state.previous_screen);
            } else {
                // If previous_screen is the same, or no specific back logic, consider exit confirmation as last resort
                self.confirm_exit();
            }
            return; 
        }

        match self.ui_state.current_screen {
            Screen::MainMenu => {
                match key_code {
                    KeyCode::Enter => {
                        let selected = self.ui_state.selected_menu_item();
                        if let Some(menu_item) = selected {
                            match menu_item.id.as_str() {
                                "install" => {
                                    log::info!("Transitioning to SystemInstallation screen...");
                                    self.start_installation_wizard();
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
                    _ => {}
                }
            },
            Screen::LanguageSelect => {
                match key_code {
                    KeyCode::Enter => {
                        if let Some(lang) = self.ui_state.selected_language() {
                            let lang_clone = lang.clone();
                            self.config.current_lang = lang;
                            if let Err(e) = crate::core::load_language(&lang_clone) {
                                log::error!("Failed to load language: {}", e);
                                self.ui_state.show_error("Language Error", &format!("Failed to load language: {}", e));
                            } else {
                                log::info!("Language changed to: {}", lang_clone);
                                
                                self.ui_state.menu_items = self.create_localized_menu_items(); // Use helper
                                
                                // Adjust keyboard layouts based on language
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
                // This block is now largely handled by the active_dialog logic at the beginning of handle_key_event.
                // Kept for context, but actions should be triggered via handle_dialog_confirm/cancel.
                // Specific key presses like 'y' or 'n' could directly trigger actions if desired,
                // but standard dialog navigation (left/right/enter/esc) is handled by the active_dialog block.
                // For now, this block will do nothing as the active_dialog logic takes precedence.
            },
            Screen::SystemInstallation => {
                // Specific key handling for SystemInstallation directly here
                match key_code {
                    KeyCode::Enter => {
                        // Only if no dialog is active, handle Enter for the installation step
                        if self.ui_state.active_dialog.is_none() {
                            if self.ui_state.installation_step_requires_text_input() {
                                // In text input: Enter confirms the input and proceeds to the next step.
                                // The current input is processed in handle_installation_next_step.
                                log::debug!("Enter pressed in text input step ({:?}), proceeding to next step.", self.ui_state.installation_step);
                                self.handle_installation_next_step(); 
                            } else {
                                // Not a text input step: Enter proceeds to the next step normally.
                                self.handle_installation_next_step(); 
                            }
                        } else {
                            // Dialog is active, Enter is handled by the dialog handler above.
                        }
                    }
                    // Beispiel: 'd' für Test-Dialog (kann später entfernt werden)
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        if self.ui_state.active_dialog.is_none() { // Nur wenn kein anderer Dialog aktiv ist
                             self.ui_state.active_dialog = Some(DialogType::YesNo {
                                title_key: "DIALOG_YESNO_EXAMPLE_TITLE".to_string(),
                                message_key: "DIALOG_YESNO_EXAMPLE_MESSAGE".to_string(),
                            });
                            self.ui_state.dialog_selected_option = 0; // Default to Yes
                        }
                    }
                    _ => {
                        // Delegate other keys to the specific input handler for the installation step
                        self.handle_installation_input(key_code);
                    }
                }
            }
        }
    }

    /// Show exit confirmation dialog
    fn confirm_exit(&mut self) {
        // If a dialog is already active, don't open another one.
        if self.ui_state.active_dialog.is_some() {
            return;
        }
        // self.ui_state.previous_screen = self.ui_state.current_screen; // NO LONGER SETTING previous_screen here
        self.ui_state.active_dialog = Some(DialogType::YesNo { 
            title_key: "LANG_CONFIRM_TITLE".to_string(),
            message_key: "LANG_EXIT_CONFIRM".to_string(),
        });
        self.ui_state.dialog_selected_option = 1; // Default to "No" (index 1)
        // self.ui_state.current_screen = Screen::ConfirmExit; // NO LONGER CHANGING current_screen
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

    fn handle_installation_next_step(&mut self) {
        log::info!("Attempting to navigate to next installation step.");

        if self.ui_state.current_installation_task_index < self.ui_state.installation_tasks.len() - 1 {
            // Mark current task as completed
            if let Some(task) = self.ui_state.installation_tasks.get_mut(self.ui_state.current_installation_task_index) {
                task.status = InstallationTaskStatus::Completed;
            }

            // Switch to the next task index
            self.ui_state.current_installation_task_index += 1;
            
            // Set the ui_state's installation_step to that of the NEW current task
            // and only THEN call update_active_task_status.
            let new_installation_step: Option<InstallationStep> = 
                if let Some(next_task_ref) = self.ui_state.installation_tasks.get(self.ui_state.current_installation_task_index) {
                    let step = Some(next_task_ref.step);
                    let next_task_title = next_task_ref.title.clone(); // Get title for logging
                    log::info!("Advancing to task: '{}' (Step: {:?})", next_task_title, step);
                    step
            } else {
                log::error!("Failed to get next task at index {}", self.ui_state.current_installation_task_index);
                // Fallback or error handling if the next task doesn't exist (should not happen)
                self.ui_state.current_installation_task_index -=1; // Back to the previous index
                if let Some(task) = self.ui_state.installation_tasks.get_mut(self.ui_state.current_installation_task_index) {
                    task.status = InstallationTaskStatus::Active; // Set previous task active again
                }
                return; // Exit function here
            };

            // Since the else-block above ends with 'return', new_installation_step is always Some here.
            // An explicit if let Some(...) is still good for readability and safety.
            if let Some(step_to_set) = new_installation_step {
                self.ui_state.installation_step = Some(step_to_set);
                self.update_active_task_status(); // Update status based on the new, set step

                // Specific actions for *entering* a new step
                if step_to_set == InstallationStep::DiskSetup {
                    self.ui_state.disk_setup_list_state.select(Some(0)); 
                    self.ui_state.disk_setup_selected_item_path = None; 
                    if let Some(info) = &self.ui_state.system_disk_info {
                        self.ui_state.current_disk_display_items = build_disk_display_list(info);
                        if !self.ui_state.current_disk_display_items.is_empty() {
                            self.ui_state.disk_setup_selected_item_path = Some(self.ui_state.current_disk_display_items[0].id_path.clone());
                        }
                    }
                }
                // Add 'else if' here for other steps that need initialization upon entry
            } else {
                 log::error!("new_installation_step was None, cannot update UI state consistently.");
            }

        } else {
            log::info!("Last installation task reached or no more tasks. Current step: {:?}", self.ui_state.installation_step);
            // Mark current (last) task as completed
            if let Some(task) = self.ui_state.installation_tasks.get_mut(self.ui_state.current_installation_task_index) {
                 task.status = InstallationTaskStatus::Completed; // Mark the last task as completed
                 // Optional: Log that the last step (e.g., Summary) has been reached and completed
                 if task.step == InstallationStep::Summary { 
                    log::info!("Summary task completed. Installation wizard finished configuration phase.");
                 }
            }
             // This is where one might switch to an "Installing" status or consider the wizard complete.
             // Currently, it remains at the last step in the list (e.g., Summary).
        }
    }

    fn handle_installation_previous_step(&mut self) {
        if self.ui_state.installation_tasks.is_empty() {
            log::warn!("handle_installation_previous_step called with empty installation_tasks. Returning to MainMenu.");
            self.ui_state.set_current_screen(Screen::MainMenu);
            self.ui_state.installation_step = None;
            self.ui_state.installation_task_list_state.select(Some(0)); // Reset selection for safety
            return;
        }

        if self.ui_state.current_installation_task_index > 0 {
            // Mark current as pending
            self.ui_state.installation_tasks[self.ui_state.current_installation_task_index].status = InstallationTaskStatus::Pending;
            
            self.ui_state.current_installation_task_index -= 1;
            
            // Mark new current as active
            self.ui_state.installation_tasks[self.ui_state.current_installation_task_index].status = InstallationTaskStatus::Active;
            self.ui_state.installation_step = Some(self.ui_state.installation_tasks[self.ui_state.current_installation_task_index].step);
            self.ui_state.installation_task_list_state.select(Some(self.ui_state.current_installation_task_index));
            self.ui_state.input_buffer.clear(); // Clear input buffer when changing steps
        } else {
            // At the first step (e.g., Welcome), current_installation_task_index is 0.
            // Going "back" should return to the main menu.
            log::debug!(
                "Current installation_task_index is 0. Transitioning to MainMenu. Current step before transition: {:?}",
                self.ui_state.installation_step
            );
            self.ui_state.set_current_screen(Screen::MainMenu); // This also sets ui_state.selected_index = 0 for MainMenu
            
            // Reset installation-specific state as we are exiting the wizard
            self.ui_state.installation_step = None; 
            self.ui_state.installation_tasks.clear();
            self.ui_state.installation_task_list_state.select(Some(0)); // Reset selection for the (now cleared) task list
        }
    }
    
    fn handle_installation_input(&mut self, key_code: KeyCode) {
        if let Some(step) = self.ui_state.installation_step {
            match step {
                InstallationStep::UserSetup => { 
                    match key_code {
                        KeyCode::Char(c) => self.ui_state.input_buffer.push(c),
                        KeyCode::Backspace => { self.ui_state.input_buffer.pop(); },
                        _ => { }
                    }
                }
                InstallationStep::DiskSetup => {
                    // Ensure current_disk_display_items is populated for navigation logic
                    if self.ui_state.current_disk_display_items.is_empty() {
                        if let Some(info) = &self.ui_state.system_disk_info {
                            self.ui_state.current_disk_display_items = build_disk_display_list(info);
                        }
                    }
                    // Now self.ui_state.current_disk_display_items should be populated (if disk_info exists)

                    let num_options = self.ui_state.current_disk_display_items.len();
                    if num_options == 0 { return; }

                    let mut current_selected_index = self.ui_state.disk_setup_list_state.selected().unwrap_or(0);

                    match key_code {
                        KeyCode::Up => {
                            if current_selected_index > 0 {
                                current_selected_index -= 1;
                            } else {
                                current_selected_index = num_options - 1; // Wrap around
                            }
                        }
                        KeyCode::Down => {
                            if current_selected_index < num_options - 1 {
                                current_selected_index += 1;
                            } else {
                                current_selected_index = 0; // Wrap around
                            }
                        }
                        _ => { return; } // Ignore other keys for now in this list
                    }
                    
                    // Loop to find the next selectable item if the direct one isn't
                    // This is a simple loop; a more robust solution might be needed for complex skip logic.
                    let initial_try_index = current_selected_index;
                    loop {
                        if let Some(item) = self.ui_state.current_disk_display_items.get(current_selected_index) {
                            if item.selectable {
                                self.ui_state.disk_setup_list_state.select(Some(current_selected_index));
                                self.ui_state.disk_setup_selected_item_path = Some(item.id_path.clone());
                                log::debug!("DiskSetup: Selected '{}' at index {}", item.id_path, current_selected_index);
                                break;
                            }
                        }
                        // Move to next/prev item and wrap around if necessary
                        if key_code == KeyCode::Up {
                            if current_selected_index > 0 { current_selected_index -= 1; } else { current_selected_index = num_options - 1; }
                        } else { // Down
                            if current_selected_index < num_options - 1 { current_selected_index += 1; } else { current_selected_index = 0; }
                        }
                        // Prevent infinite loop if no items are selectable (should not happen with current dummy data)
                        if current_selected_index == initial_try_index { 
                            log::warn!("DiskSetup: No selectable item found in the direction of navigation.");
                            break; 
                        }
                    }
                }
                _ => {}
            }
        } else {
            log::warn!("handle_installation_input called without an active installation_step");
        }
    }

    fn update_active_task_status(&mut self) {
        // First, set all tasks to Pending, except those already Completed.
        // This prevents an old task from remaining Active if something goes wrong.
        for task in self.ui_state.installation_tasks.iter_mut() {
            if task.status == InstallationTaskStatus::Active {
                task.status = InstallationTaskStatus::Pending; 
            }
        }
    
        // Then mark the current task as Active.
        if let Some(task_to_activate) = self.ui_state.installation_tasks.get_mut(self.ui_state.current_installation_task_index) {
            // Check if the task's step matches the current installation_step.
            // This is an additional safety check.
            if Some(task_to_activate.step) == self.ui_state.installation_step {
                task_to_activate.status = InstallationTaskStatus::Active;
                self.ui_state.installation_task_list_state.select(Some(self.ui_state.current_installation_task_index));
                log::debug!("Task '{}' at index {} set to Active for step {:?}", 
                           task_to_activate.title, self.ui_state.current_installation_task_index, self.ui_state.installation_step);
            } else {
                log::warn!("Mismatch in update_active_task_status: Task step {:?} at index {} does not match current_installation_step {:?}. Attempting to find and activate correct task.", 
                           task_to_activate.step, self.ui_state.current_installation_task_index, self.ui_state.installation_step);
                // Try to find and activate the correct task for the current installation_step
                let mut found_and_activated = false;
                for (index, task) in self.ui_state.installation_tasks.iter_mut().enumerate() {
                    if Some(task.step) == self.ui_state.installation_step {
                        task.status = InstallationTaskStatus::Active;
                        self.ui_state.current_installation_task_index = index; // Correct the index
                        self.ui_state.installation_task_list_state.select(Some(index));
                        log::debug!("Corrected: Task '{}' at index {} set to Active for step {:?}", task.title, index, self.ui_state.installation_step);
                        found_and_activated = true;
                        break;
                    }
                }
                if !found_and_activated {
                    log::error!("Could not find a matching task for current_installation_step {:?} to activate.", self.ui_state.installation_step);
                }
            }
        } else {
            log::warn!("update_active_task_status: current_installation_task_index {} is out of bounds for installation_tasks (len {}).", 
                       self.ui_state.current_installation_task_index, self.ui_state.installation_tasks.len());
        }
    }

    fn handle_dialog_confirm(&mut self) {
        if let Some(dialog_type) = self.ui_state.active_dialog.clone() { // Clone to avoid borrow issues
            match dialog_type {
                DialogType::YesNo { ref title_key, .. } => {
                    let choice_is_yes = self.ui_state.dialog_selected_option == 0;
                    log::info!("Dialog confirmed. Choice: {}, Title Key: {}", if choice_is_yes { "Yes" } else { "No" }, title_key);

                    if title_key == "LANG_CONFIRM_TITLE" { // Specific logic for the Exit Confirmation Dialog
                        if choice_is_yes {
                            self.should_quit = true;
                        } else {
                            self.ui_state.active_dialog = None; 
                        }
                    } else {
                        log::warn!("Unhandled YesNo dialog confirmation for title_key: {}", title_key);
                        self.ui_state.active_dialog = None; 
                    }
                }
                DialogType::ThemeSelector => {
                    // The confirmation for ThemeSelector is handled by handle_theme_dialog_confirm.
                    // Do nothing here to avoid duplicate logic.
                    // handle_dialog_confirm would simply close the dialog, which is okay, but handle_theme_dialog_confirm is more specific.
                    // We could close the dialog here, but it's cleaner to do it in the specific function.
                    // Instead, handle_theme_dialog_confirm is called directly on Enter in the ThemeSelector dialog.
                    log::debug!("ThemeSelector dialog confirmation is handled by handle_theme_dialog_confirm.");
                }
            }
        }
    }

    fn handle_dialog_cancel(&mut self) {
        if let Some(dialog_type) = &self.ui_state.active_dialog {
            log::info!("Dialog cancelled. Dialog type: {:?}", dialog_type);
        }
        self.ui_state.active_dialog = None; 
    }

    // Specific confirmation function for the Theme Selection dialog
    fn handle_theme_dialog_confirm(&mut self) {
        if let Some(DialogType::ThemeSelector) = self.ui_state.active_dialog {
            if self.ui_state.dialog_selected_option < self.ui_state.themes.len() {
                self.ui_state.active_theme_index = self.ui_state.dialog_selected_option;
                log::info!("Theme selected: {}", self.ui_state.themes[self.ui_state.active_theme_index].name);
            } else {
                log::warn!("Theme selection index out of bounds: {}", self.ui_state.dialog_selected_option);
            }
        }
        self.ui_state.active_dialog = None; // Close dialog
    }

    fn start_installation_wizard(&mut self) {
        self.ui_state.installation_step = Some(InstallationStep::Welcome); // Start with Welcome
        self.ui_state.current_installation_task_index = 0; // Ensure task index is reset
        self.ui_state.installation_tasks = self.initialize_installation_tasks();
        self.ui_state.installation_task_list_state.select(Some(0)); // Select first task in list
        self.update_active_task_status(); // Set Welcome task to Active
        self.ui_state.set_current_screen(Screen::SystemInstallation);
    }

    fn initialize_installation_tasks(&self) -> Vec<InstallationTaskItem> {
        vec![
            InstallationTaskItem { id: "welcome".to_string(), title: get_text("TASK_WELCOME"), step: InstallationStep::Welcome, status: InstallationTaskStatus::Active },
            InstallationTaskItem { id: "disk_setup".to_string(), title: get_text("TASK_DISK_SETUP"), step: InstallationStep::DiskSetup, status: InstallationTaskStatus::Pending },
            InstallationTaskItem { id: "user_setup".to_string(), title: get_text("TASK_USER_SETUP"), step: InstallationStep::UserSetup, status: InstallationTaskStatus::Pending },
            InstallationTaskItem { id: "summary".to_string(), title: get_text("TASK_SUMMARY"), step: InstallationStep::Summary, status: InstallationTaskStatus::Pending },
        ]
    }

    // Helper function to create localized menu items
    fn create_localized_menu_items(&self) -> Vec<MenuItem> {
        vec![
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
        ]
    }
}