use crate::commands::{Command, CommandResult};
use crate::error::FugaError;
use crate::traits::{ConfigRepository, FileSystemService, UIService};

/// Actions supported by the preset command
#[derive(Debug, PartialEq, Eq)]
pub enum PresetAction {
    Save { name: String },
    Load { name: String },
    List,
    Show { name: String },
    Delete { name: String },
}

/// Command implementation for managing presets
pub struct PresetCommand<'a> {
    config_repo: &'a dyn ConfigRepository,
    fs_service: &'a dyn FileSystemService,
    ui_service: &'a dyn UIService,
    action: PresetAction,
}

impl<'a> PresetCommand<'a> {
    pub fn new(
        config_repo: &'a dyn ConfigRepository,
        fs_service: &'a dyn FileSystemService,
        ui_service: &'a dyn UIService,
        action: PresetAction,
    ) -> Self {
        Self {
            config_repo,
            fs_service,
            ui_service,
            action,
        }
    }

    fn save(&self, name: &str) -> CommandResult {
        let targets = self.config_repo.get_marked_targets()?;
        self.config_repo.save_preset(name, &targets)?;
        println!(
            "✅ : Preset '{}' saved with {} target(s).",
            name,
            targets.len()
        );
        Ok(())
    }

    fn load(&self, name: &str) -> CommandResult {
        let preset = self
            .config_repo
            .get_preset(name)?
            .ok_or_else(|| FugaError::OperationFailed(format!("Preset '{}' not found.", name)))?;
        self.config_repo.set_marked_targets(&preset)?;
        println!(
            "✅ : Preset '{}' loaded. Mark list now tracks {} target(s).",
            name,
            preset.len()
        );
        Ok(())
    }

    fn list(&self) -> CommandResult {
        let presets = self.config_repo.list_presets()?;
        if presets.is_empty() {
            println!(
                "{} : No presets saved.",
                self.ui_service.get_icon_information()
            );
            return Ok(());
        }

        println!(
            "{} : Saved presets:",
            self.ui_service.get_icon_information()
        );
        for name in presets {
            println!("- {}", name);
        }
        Ok(())
    }

    fn show(&self, name: &str) -> CommandResult {
        let preset = self
            .config_repo
            .get_preset(name)?
            .ok_or_else(|| FugaError::OperationFailed(format!("Preset '{}' not found.", name)))?;

        println!(
            "{} : Preset '{}':",
            self.ui_service.get_icon_information(),
            name
        );

        for path in preset {
            let icon = self
                .ui_service
                .get_icon_for_target_type(self.fs_service.get_file_type(&path));
            println!("{} {}", icon, path);
        }

        Ok(())
    }

    fn delete(&self, name: &str) -> CommandResult {
        if !self.config_repo.delete_preset(name)? {
            return Err(FugaError::OperationFailed(format!(
                "Preset '{}' not found.",
                name
            )));
        }

        println!("✅ : Preset '{}' deleted.", name);
        Ok(())
    }
}

impl<'a> Command for PresetCommand<'a> {
    fn execute(&self) -> CommandResult {
        match &self.action {
            PresetAction::Save { name } => self.save(name),
            PresetAction::Load { name } => self.load(name),
            PresetAction::List => self.list(),
            PresetAction::Show { name } => self.show(name),
            PresetAction::Delete { name } => self.delete(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::error::FugaResult;
    use crate::fuga::TargetType;
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct StubConfigRepository {
        marks: RefCell<Vec<String>>,
        presets: RefCell<BTreeMap<String, Vec<String>>>,
    }

    impl StubConfigRepository {
        fn with_marks(marks: &[&str]) -> Self {
            Self {
                marks: RefCell::new(marks.iter().map(|m| m.to_string()).collect()),
                presets: RefCell::default(),
            }
        }

        fn set_preset(&self, name: &str, values: &[&str]) {
            self.presets.borrow_mut().insert(
                name.to_string(),
                values.iter().map(|v| v.to_string()).collect(),
            );
        }

        fn current_marks(&self) -> Vec<String> {
            self.marks.borrow().clone()
        }

        fn preset_contents(&self, name: &str) -> Option<Vec<String>> {
            self.presets.borrow().get(name).cloned()
        }
    }

    impl ConfigRepository for StubConfigRepository {
        fn load_config(&self) -> FugaResult<AppConfig> {
            Ok(AppConfig::default())
        }

        fn store_config(&self, _config: &AppConfig) -> FugaResult<()> {
            Ok(())
        }

        fn set_marked_targets(&self, targets: &[String]) -> FugaResult<()> {
            *self.marks.borrow_mut() = targets.to_vec();
            Ok(())
        }

        fn get_marked_targets(&self) -> FugaResult<Vec<String>> {
            Ok(self.marks.borrow().clone())
        }

        fn reset_marks(&self) -> FugaResult<()> {
            self.marks.borrow_mut().clear();
            Ok(())
        }

        fn list_presets(&self) -> FugaResult<Vec<String>> {
            Ok(self.presets.borrow().keys().cloned().collect())
        }

        fn get_preset(&self, name: &str) -> FugaResult<Option<Vec<String>>> {
            Ok(self.presets.borrow().get(name).cloned())
        }

        fn save_preset(&self, name: &str, targets: &[String]) -> FugaResult<()> {
            self.presets
                .borrow_mut()
                .insert(name.to_string(), targets.to_vec());
            Ok(())
        }

        fn delete_preset(&self, name: &str) -> FugaResult<bool> {
            Ok(self.presets.borrow_mut().remove(name).is_some())
        }
    }

    #[derive(Default)]
    struct StubFileSystemService;

    impl FileSystemService for StubFileSystemService {
        fn get_file_info(&self, _path: &str) -> FugaResult<crate::fuga::FileInfo> {
            unimplemented!("not required for preset tests")
        }

        fn get_abs_path(&self, path: &str) -> FugaResult<String> {
            Ok(path.to_string())
        }

        fn get_file_type(&self, _path: &str) -> TargetType {
            TargetType::File
        }

        fn copy_items(&self, _src: &str, _dst: &str) -> FugaResult<()> {
            unimplemented!()
        }

        fn move_items(&self, _src: &str, _dst: &str) -> FugaResult<()> {
            unimplemented!()
        }

        fn link_items(&self, _src: &str, _dst: &str) -> FugaResult<()> {
            unimplemented!()
        }
    }

    #[derive(Default)]
    struct StubUIService;

    impl UIService for StubUIService {
        fn get_colorized_text(&self, text: &str, _is_bold: bool) -> String {
            text.to_string()
        }

        fn get_icon_information(&self) -> String {
            "[i]".to_string()
        }

        fn get_icon_for_target_type(&self, _target_type: TargetType) -> String {
            "[FILE]".to_string()
        }
    }

    fn make_command<'a>(
        repo: &'a dyn ConfigRepository,
        fs: &'a dyn FileSystemService,
        ui: &'a dyn UIService,
        action: PresetAction,
    ) -> PresetCommand<'a> {
        PresetCommand::new(repo, fs, ui, action)
    }

    #[test]
    fn save_preset_persists_current_marks() {
        let repo = StubConfigRepository::with_marks(&["/abs/A", "/abs/B"]);
        let fs = StubFileSystemService;
        let ui = StubUIService;

        let command = make_command(
            &repo,
            &fs,
            &ui,
            PresetAction::Save {
                name: "template".to_string(),
            },
        );

        command.execute().expect("save should succeed");

        assert_eq!(
            repo.preset_contents("template").unwrap(),
            vec!["/abs/A".to_string(), "/abs/B".to_string()]
        );
    }

    #[test]
    fn load_preset_replaces_marks() {
        let repo = StubConfigRepository::with_marks(&["/abs/initial"]);
        repo.set_preset("template", &["/abs/A", "/abs/B"]);
        let fs = StubFileSystemService;
        let ui = StubUIService;

        let command = make_command(
            &repo,
            &fs,
            &ui,
            PresetAction::Load {
                name: "template".to_string(),
            },
        );

        command.execute().expect("load should succeed");

        assert_eq!(
            repo.current_marks(),
            vec!["/abs/A".to_string(), "/abs/B".to_string()]
        );
    }

    #[test]
    fn load_missing_preset_returns_error() {
        let repo = StubConfigRepository::with_marks(&[]);
        let fs = StubFileSystemService;
        let ui = StubUIService;

        let command = make_command(
            &repo,
            &fs,
            &ui,
            PresetAction::Load {
                name: "missing".to_string(),
            },
        );

        let error = command.execute().expect_err("missing preset should error");
        assert!(matches!(
            error,
            FugaError::OperationFailed(message) if message.contains("missing")
        ));
    }

    #[test]
    fn delete_missing_preset_returns_error() {
        let repo = StubConfigRepository::with_marks(&[]);
        let fs = StubFileSystemService;
        let ui = StubUIService;

        let command = make_command(
            &repo,
            &fs,
            &ui,
            PresetAction::Delete {
                name: "missing".to_string(),
            },
        );

        let error = command.execute().expect_err("missing delete should error");
        assert!(matches!(
            error,
            FugaError::OperationFailed(message) if message.contains("missing")
        ));
    }

    #[test]
    fn list_presets_succeeds_when_empty() {
        let repo = StubConfigRepository::with_marks(&[]);
        let fs = StubFileSystemService;
        let ui = StubUIService;

        let command = make_command(&repo, &fs, &ui, PresetAction::List);
        assert!(command.execute().is_ok());
    }
}
