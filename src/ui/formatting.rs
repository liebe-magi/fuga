use crate::fuga::TargetType;
use crate::traits::UIService;
use termion::{color, style};

#[derive(Default)]
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

    fn get_icon_information(&self) -> String {
        match emojis::get_by_shortcode("information_source") {
            Some(emoji) => format!("{emoji} "),
            None => "ℹ️ ".to_string(), // Fallback emoji
        }
    }

    fn get_icon_for_target_type(&self, target_type: TargetType) -> String {
        match target_type {
            TargetType::File => "📄".to_string(),
            TargetType::Dir => "📁".to_string(),
            TargetType::None => "❌".to_string(),
        }
    }
}
