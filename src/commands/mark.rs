use crate::commands::{Command, CommandResult};
use crate::fuga::TargetType;
use crate::traits::{ConfigRepository, FileSystemService, UIService};

/// Mark command for setting, showing, or resetting marked paths
pub struct MarkCommand<'a> {
    config_repo: &'a dyn ConfigRepository,
    fs_service: &'a dyn FileSystemService,
    ui_service: &'a dyn UIService,
    action: MarkAction,
}

#[derive(Debug)]
pub enum MarkAction {
    SetTarget(String),
    ShowTarget,
    ResetTarget,
}

impl<'a> MarkCommand<'a> {
    pub fn new(
        config_repo: &'a dyn ConfigRepository,
        fs_service: &'a dyn FileSystemService,
        ui_service: &'a dyn UIService,
        action: MarkAction,
    ) -> Self {
        Self {
            config_repo,
            fs_service,
            ui_service,
            action,
        }
    }

    fn get_icon_information(&self) -> String {
        format!(
            "{} ",
            emojis::get_by_shortcode("information_source").unwrap()
        )
    }

    fn get_file_type(&self, path: &str) -> TargetType {
        match self.fs_service.get_file_info(path) {
            Ok(info) => {
                if !info.exists {
                    TargetType::None
                } else if info.is_file {
                    TargetType::File
                } else {
                    TargetType::Dir
                }
            }
            Err(_) => TargetType::None,
        }
    }

    fn get_icon(&self, path: &str) -> String {
        match self.get_file_type(path) {
            TargetType::File => "📄".to_string(),
            TargetType::Dir => "📁".to_string(),
            TargetType::None => "❌".to_string(),
        }
    }
}

impl<'a> Command for MarkCommand<'a> {
    fn execute(&self) -> CommandResult {
        match &self.action {
            MarkAction::SetTarget(target) => match self.get_file_type(target) {
                TargetType::None => {
                    println!(
                        "❌ : {} is not found.",
                        self.ui_service.get_colorized_text(target, true)
                    );
                    Ok(())
                }
                _ => {
                    let abs_path = self.fs_service.get_abs_path(target)?;
                    self.config_repo.store_path(&abs_path)?;
                    println!(
                        "✅ : {} {} has marked.",
                        self.get_icon(target),
                        self.ui_service.get_colorized_text(target, true)
                    );
                    Ok(())
                }
            },
            MarkAction::ShowTarget => {
                let target = self.config_repo.get_marked_path()?;
                if target.is_empty() {
                    println!("{} : No path has been marked.", self.get_icon_information());
                } else {
                    match self.get_file_type(&target) {
                        TargetType::None => {
                            println!("{} : ❓ {}", self.get_icon_information(), target)
                        }
                        _ => println!(
                            "{} : {} {}",
                            self.get_icon_information(),
                            self.get_icon(&target),
                            target
                        ),
                    }
                }
                Ok(())
            }
            MarkAction::ResetTarget => {
                self.config_repo.reset_mark()?;
                println!("✅ : The marked path has reset.");
                Ok(())
            }
        }
    }
}
