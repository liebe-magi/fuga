use crate::fuga::TargetType;
use crate::traits::UIService;
use crossterm::style::{Color, Stylize};
use std::env;

pub struct TerminalUIService {
    use_emoji: bool,
}

impl TerminalUIService {
    pub fn new() -> Self {
        let disabled = env::var("FUGA_DISABLE_EMOJI")
            .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "True"))
            .unwrap_or(false);

        Self {
            use_emoji: !disabled,
        }
    }

    fn colorize(&self, text: &str, is_bold: bool) -> String {
        let styled = if is_bold {
            text.bold().with(Color::Green)
        } else {
            text.with(Color::Green)
        };

        styled.to_string()
    }
}

impl UIService for TerminalUIService {
    fn get_colorized_text(&self, text: &str, is_bold: bool) -> String {
        self.colorize(text, is_bold)
    }

    fn get_icon_information(&self) -> String {
        if self.use_emoji {
            if let Some(emoji) = emojis::get_by_shortcode("information_source") {
                format!("{emoji} ")
            } else {
                "ℹ️ ".to_string()
            }
        } else {
            "[i] ".to_string()
        }
    }

    fn get_icon_for_target_type(&self, target_type: TargetType) -> String {
        match (self.use_emoji, target_type) {
            (true, TargetType::File) => "📄".to_string(),
            (true, TargetType::Dir) => "📁".to_string(),
            (true, TargetType::None) => "❌".to_string(),
            (false, TargetType::File) => "[FILE]".to_string(),
            (false, TargetType::Dir) => "[DIR]".to_string(),
            (false, TargetType::None) => "[ERR]".to_string(),
        }
    }
}

impl Default for TerminalUIService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emoji_icons_toggle_via_env() {
        std::env::set_var("FUGA_DISABLE_EMOJI", "1");
        let ui = TerminalUIService::new();
        assert_eq!(ui.get_icon_for_target_type(TargetType::File), "[FILE]");
        assert_eq!(ui.get_icon_information().trim(), "[i]");
        std::env::remove_var("FUGA_DISABLE_EMOJI");

        let ui = TerminalUIService::new();
        assert_eq!(ui.get_icon_for_target_type(TargetType::File), "📄");
    }
}
