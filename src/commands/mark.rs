use crate::commands::{Command, CommandResult};
use crate::error::FugaError;
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
    Set(String),
    Show,
    Reset,
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
}

impl<'a> Command for MarkCommand<'a> {
    fn execute(&self) -> CommandResult {
        match &self.action {
            MarkAction::Set(target) => {
                let info = self.fs_service.get_file_info(target)?;

                if !info.exists {
                    return Err(FugaError::FileNotFound(target.clone()));
                }

                let abs_path = self.fs_service.get_abs_path(target)?;
                self.config_repo.store_path(&abs_path)?;

                let target_type = if info.is_file {
                    TargetType::File
                } else {
                    TargetType::Dir
                };

                println!(
                    "✅ : {} {} has marked.",
                    self.ui_service.get_icon_for_target_type(target_type),
                    self.ui_service.get_colorized_text(target, true)
                );
                Ok(())
            }
            MarkAction::Show => {
                let target = self.config_repo.get_marked_path()?;
                if target.is_empty() {
                    println!(
                        "{} : No path has been marked.",
                        self.ui_service.get_icon_information()
                    );
                } else {
                    let target_type = self.fs_service.get_file_type(&target);
                    match target_type {
                        TargetType::None => {
                            println!("{} : ❓ {}", self.ui_service.get_icon_information(), target)
                        }
                        _ => println!(
                            "{} : {} {}",
                            self.ui_service.get_icon_information(),
                            self.ui_service.get_icon_for_target_type(target_type),
                            target
                        ),
                    }
                }
                Ok(())
            }
            MarkAction::Reset => {
                self.config_repo.reset_mark()?;
                println!("✅ : The marked path has reset.");
                Ok(())
            }
        }
    }
}
