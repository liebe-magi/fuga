use crate::config::AppConfig;
use crate::error::{FugaError, FugaResult};
use crate::fuga::APP_NAME;
use crate::traits::ConfigRepository;
use dirs::config_dir;
use std::path::PathBuf;

pub struct FileConfigRepository;

impl FileConfigRepository {
    pub fn new() -> Self {
        Self
    }

    fn get_config_path() -> Option<PathBuf> {
        config_dir()
    }
}

impl ConfigRepository for FileConfigRepository {
    fn load_config(&self) -> FugaResult<AppConfig> {
        let mut config = confy::load::<AppConfig>(APP_NAME, APP_NAME)?;

        if config.user_config.box_path.is_empty() {
            let config_path = Self::get_config_path().ok_or(FugaError::ConfigPathMissing)?;

            config.user_config.box_path = format!("{}", config_path.join("box").display());
            self.store_config(&config)?;
        }

        Ok(config)
    }

    fn store_config(&self, config: &AppConfig) -> FugaResult<()> {
        confy::store(APP_NAME, APP_NAME, config)?;
        Ok(())
    }

    fn store_path(&self, target: &str) -> FugaResult<()> {
        let mut config = self.load_config()?;
        config.data.target = target.to_string();
        self.store_config(&config)?;
        Ok(())
    }

    fn get_marked_path(&self) -> FugaResult<String> {
        let config = self.load_config()?;
        Ok(config.data.target)
    }

    fn reset_mark(&self) -> FugaResult<()> {
        let mut config = self.load_config()?;
        config.data.target = "".to_string();
        self.store_config(&config)?;
        Ok(())
    }
}
