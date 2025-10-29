use crate::config::AppConfig;
use crate::error::{FugaError, FugaResult};
use crate::fuga::APP_NAME;
use crate::traits::ConfigRepository;
use dirs::config_dir;
use std::path::PathBuf;

#[derive(Default)]
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

    fn set_marked_targets(&self, targets: &[String]) -> FugaResult<()> {
        let mut config = self.load_config()?;
        config.data.targets = targets.to_vec();
        config.data.target = None;
        self.store_config(&config)
    }

    fn get_marked_targets(&self) -> FugaResult<Vec<String>> {
        let mut config = self.load_config()?;
        let mut mutated = false;

        // Remove empty strings that may be left over from legacy state
        if config
            .data
            .targets
            .iter()
            .any(|value| value.trim().is_empty())
        {
            config.data.targets.retain(|value| !value.trim().is_empty());
            mutated = true;
        }

        if !config.data.targets.is_empty() {
            if config.data.target.is_some() {
                config.data.target = None;
                mutated = true;
            }

            if mutated {
                self.store_config(&config)?;
            }

            return Ok(config.data.targets.clone());
        }

        let legacy = config
            .data
            .target
            .take()
            .filter(|legacy| !legacy.trim().is_empty());

        if let Some(legacy_target) = legacy {
            config.data.targets = vec![legacy_target];
            mutated = true;
        }

        if mutated {
            self.store_config(&config)?;
        }

        Ok(config.data.targets.clone())
    }

    fn reset_marks(&self) -> FugaResult<()> {
        self.set_marked_targets(&[])
    }
}
