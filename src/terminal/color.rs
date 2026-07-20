use colored::*;

/// Terminal color utility
pub struct Color;

impl Color {
    /// Success message (Green)
    pub fn success(text: &str) -> ColoredString {
        text.green().bold()
    }

    /// Error message (Red)
    pub fn error(text: &str) -> ColoredString {
        text.red().bold()
    }

    /// Warning message (Yellow)
    pub fn warning(text: &str) -> ColoredString {
        text.yellow().bold()
    }

    /// Information (Blue)
    pub fn info(text: &str) -> ColoredString {
        text.blue().bold()
    }

    /// Title (Cyan)
    pub fn title(text: &str) -> ColoredString {
        text.cyan().bold()
    }

    /// Highlight (Magenta)
    pub fn highlight(text: &str) -> ColoredString {
        text.magenta().bold()
    }

    /// Neutral (White)
    pub fn normal(text: &str) -> ColoredString {
        text.white()
    }

    /// Critical Alert
    pub fn critical(text: &str) -> ColoredString {
        text.bright_red().bold().underline()
    }

    /// Drone ID
    pub fn drone(text: &str) -> ColoredString {
        text.bright_blue().bold()
    }

    /// AFV Status
    pub fn afv(text: &str) -> ColoredString {
        text.bright_green().bold()
    }

    /// Threat Level
    pub fn threat(text: &str) -> ColoredString {
        text.bright_red().bold()
    }

    /// Countermeasure
    pub fn countermeasure(text: &str) -> ColoredString {
        text.bright_yellow().bold()
    }

    /// Dashboard Border
    pub fn border(text: &str) -> ColoredString {
        text.bright_cyan()
    }
}