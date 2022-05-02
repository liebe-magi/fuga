use dirs::config_dir;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use termion::{color, style};

pub const APP_NAME: &str = "fugue";
pub const VERSION: &str = "v0.0.1";

/// The struct of the config file
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub user_config: UserConfig,
    pub data: Data,
}

/// The struct of the user config
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub box_path: String,
}

/// The struct of data
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Data {
    pub temp_list: Vec<String>,
}

/// Get the path of the config file
fn get_config_path() -> PathBuf {
    return config_dir().unwrap();
}

/// Get the bold text
fn get_bold_text(text: &str) -> String {
    return format!("{}{}{}", style::Bold, text, style::Reset);
}

/// Get the colorized text
pub fn get_colorized_text(text: &str, is_bold: bool) -> String {
    match is_bold {
        true => format!(
            "{}{}{}",
            color::Fg(color::LightGreen),
            get_bold_text(text),
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

/// load AppConfig
pub fn load_config() -> Result<AppConfig, confy::ConfyError> {
    match confy::load::<AppConfig>("fugue") {
        Ok(mut config) => match config.user_config.box_path.is_empty() {
            true => {
                let config_path = get_config_path();
                config.user_config.box_path = format!("{}", config_path.join("box").display());
                match confy::store("fugue", &config) {
                    Ok(_) => Ok(config),
                    Err(e) => Err(e),
                }
            }
            false => Ok(config),
        },
        Err(err) => Err(err),
    }
}

/// Get version of this tool.
pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let version_text = format!("v{}", version);
    return version_text;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_version() {
        let version = super::get_version();
        assert_eq!(version, super::VERSION);
    }
}
