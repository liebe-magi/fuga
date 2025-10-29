use crate::commands::{Command, CommandResult};
use crate::error::FugaError;
use crate::fuga::TargetType;
use crate::traits::{ConfigRepository, FileSystemService, PathService, UIService};

/// Copy command for copying marked files/directories
pub struct CopyCommand<'a> {
    config_repo: &'a dyn ConfigRepository,
    fs_service: &'a dyn FileSystemService,
    ui_service: &'a dyn UIService,
    path_service: &'a dyn PathService,
    destination: Option<String>,
}

impl<'a> CopyCommand<'a> {
    pub fn new(
        config_repo: &'a dyn ConfigRepository,
        fs_service: &'a dyn FileSystemService,
        ui_service: &'a dyn UIService,
        path_service: &'a dyn PathService,
        destination: Option<String>,
    ) -> Self {
        Self {
            config_repo,
            fs_service,
            ui_service,
            path_service,
            destination,
        }
    }
}

impl<'a> Command for CopyCommand<'a> {
    fn execute(&self) -> CommandResult {
        let targets = self.config_repo.get_marked_targets()?;
        if targets.is_empty() {
            return Err(FugaError::OperationFailed("No targets marked.".to_string()));
        }

        let destination_arg = self.destination.as_deref();
        let destination_accepts_many = match destination_arg {
            Some(dest) => {
                let info = self.fs_service.get_file_info(dest)?;
                info.exists && info.is_dir
            }
            None => false,
        };

        if targets.len() > 1 && destination_arg.is_some() && !destination_accepts_many {
            return Err(FugaError::OperationFailed(
                "Cannot copy multiple items to a single file path.".to_string(),
            ));
        }

        for target in targets {
            let info = self.fs_service.get_file_info(&target)?;
            if !info.exists {
                return Err(FugaError::FileNotFound(target));
            }

            let target_type = if info.is_file {
                TargetType::File
            } else {
                TargetType::Dir
            };

            let dst_name = self.path_service.get_destination_name_with_info(
                &target,
                &info,
                destination_arg,
                self.fs_service,
            )?;

            println!(
                "{} : Copying {} {} -> {}",
                self.ui_service.get_icon_information(),
                self.ui_service.get_icon_for_target_type(target_type),
                self.ui_service.get_colorized_text(&target, true),
                self.ui_service.get_colorized_text(&dst_name, true)
            );

            self.fs_service.copy_items(&target, &dst_name)?;

            let dst_type = self.fs_service.get_file_type(&dst_name);
            println!(
                "✅ : {} {} copied.",
                self.ui_service.get_icon_for_target_type(dst_type),
                self.ui_service.get_colorized_text(&dst_name, true)
            );
        }

        Ok(())
    }
}
