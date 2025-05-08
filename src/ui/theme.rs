use tui::style::{Color, Style};

/// UI theme
#[derive(Debug, Clone)]
pub struct Theme {
    /// Background color
    pub background: Color,
    /// Text color
    pub text: Color,
    /// Title color
    pub title: Color,
    /// Highlight color
    pub highlight: Color,
    /// Secondary highlight color
    pub highlight_secondary: Color,
    /// Warning color
    pub warning: Color,
    /// Error color
    pub error: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Black,
            text: Color::White,
            title: Color::Green,
            highlight: Color::Green,
            highlight_secondary: Color::Yellow,
            warning: Color::Yellow,
            error: Color::Red,
        }
    }
}

impl Theme {
    /// Get a theme by name
    pub fn get(name: &str) -> Self {
        match name {
            "dark" => Self::dark(),
            "light" => Self::light(),
            "blue" => Self::blue(),
            _ => Self::default(),
        }
    }

    /// Dark theme
    pub fn dark() -> Self {
        Self {
            background: Color::Black,
            text: Color::White,
            title: Color::Green,
            highlight: Color::Green,
            highlight_secondary: Color::Yellow,
            warning: Color::Yellow,
            error: Color::Red,
        }
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            background: Color::White,
            text: Color::Black,
            title: Color::Blue,
            highlight: Color::Blue,
            highlight_secondary: Color::Magenta,
            warning: Color::Yellow,
            error: Color::Red,
        }
    }

    /// Blue theme
    pub fn blue() -> Self {
        Self {
            background: Color::Rgb(16, 24, 48),
            text: Color::White,
            title: Color::Cyan,
            highlight: Color::Cyan,
            highlight_secondary: Color::Rgb(255, 165, 0), // Orange
            warning: Color::Yellow,
            error: Color::Red,
        }
    }

    /// Get normal text style
    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text).bg(self.background)
    }

    /// Get title style
    pub fn title_style(&self) -> Style {
        Style::default().fg(self.title).bg(self.background)
    }

    /// Get highlight style
    pub fn highlight_style(&self) -> Style {
        Style::default().fg(self.highlight).bg(self.background)
    }

    /// Get secondary highlight style
    pub fn highlight_secondary_style(&self) -> Style {
        Style::default().fg(self.highlight_secondary).bg(self.background)
    }

    /// Get warning style
    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.warning).bg(self.background)
    }

    /// Get error style
    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error).bg(self.background)
    }
}