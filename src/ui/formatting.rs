use crate::traits::UIService;
use termion::{color, style};

pub struct TerminalUIService;

impl TerminalUIService {
    pub fn new() -> Self {
        Self
    }

    fn get_bold_text(&self, text: &str) -> String {
        format!("{}{}{}", style::Bold, text, style::Reset)
    }
}

impl UIService for TerminalUIService {
    fn get_colorized_text(&self, text: &str, is_bold: bool) -> String {
        match is_bold {
            true => format!(
                "{}{}{}",
                color::Fg(color::LightGreen),
                self.get_bold_text(text),
                color::Fg(color::Reset)
            ),
            false => format!(
                "{}{}{}",
                color::Fg(color::LightGreen),
                text,
                color::Fg(color::Reset)
            ),
        }
    }
}
