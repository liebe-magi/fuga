use crate::commands::{Command, CommandResult};
use crate::error::FugaError;
use crate::traits::{ConfigRepository, FileSystemService, PathService, UIService};

/// Move command for moving marked files/directories
pub struct MoveCommand<'a> {
    config_repo: &'a dyn ConfigRepository,
    fs_service: &'a dyn FileSystemService,
    ui_service: &'a dyn UIService,
    path_service: &'a dyn PathService,
    name: Option<String>,
}

impl<'a> MoveCommand<'a> {
    pub fn new(
        config_repo: &'a dyn ConfigRepository,
        fs_service: &'a dyn FileSystemService,
        ui_service: &'a dyn UIService,
        path_service: &'a dyn PathService,
        name: Option<String>,
    ) -> Self {
        Self {
            config_repo,
            fs_service,
            ui_service,
            path_service,
            name,
        }
    }
}

impl<'a> Command for MoveCommand<'a> {
    fn execute(&self) -> CommandResult {
        let target = self.config_repo.get_marked_path()?;

        // Check if target exists and get file info
        match self.fs_service.get_file_info(&target) {
            Ok(file_info) if file_info.exists => {
                let dst_name = self.path_service.get_destination_name_with_info(
                    &target,
                    &file_info,
                    self.name.clone(),
                    self.fs_service,
                );

                let target_type = self.fs_service.get_file_type(&target);
                println!(
                    "{} : Start moving {} {} from {}",
                    self.ui_service.get_icon_information(),
                    self.ui_service.get_icon_for_target_type(target_type),
                    self.ui_service.get_colorized_text(&dst_name, true),
                    target
                );

                // Perform the move operation
                self.fs_service.move_items(&target, &dst_name)?;

                let dst_type = self.fs_service.get_file_type(&dst_name);
                println!(
                    "✅ : {} {} has been moved.",
                    self.ui_service.get_icon_for_target_type(dst_type),
                    self.ui_service.get_colorized_text(&dst_name, true)
                );

                // Move operation resets the mark
                self.config_repo.reset_mark()?;

                Ok(())
            }
            Ok(_) => {
                // File doesn't exist
                if target.is_empty() {
                    Err(FugaError::OperationFailed(
                        "No path has been marked".to_string(),
                    ))
                } else {
                    Err(FugaError::FileNotFound(target))
                }
            }
            Err(e) => Err(e),
        }
    }
}
