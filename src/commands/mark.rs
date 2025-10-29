use crate::commands::{Command, CommandResult};
use crate::error::{FugaError, FugaResult};
use crate::fuga::{FileInfo, TargetType};
use crate::traits::{ConfigRepository, FileSystemService, UIService};
use std::collections::HashSet;

/// Mark command for managing marked paths
pub struct MarkCommand<'a> {
    config_repo: &'a dyn ConfigRepository,
    fs_service: &'a dyn FileSystemService,
    ui_service: &'a dyn UIService,
    action: MarkAction,
}

#[derive(Debug)]
pub enum MarkAction {
    Set(Vec<String>),
    Add(Vec<String>),
    List,
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
            MarkAction::Set(paths) => self.set_targets(paths),
            MarkAction::Add(paths) => self.add_targets(paths),
            MarkAction::List => self.list_targets(),
            MarkAction::Reset => self.reset_targets(),
        }
    }
}

impl<'a> MarkCommand<'a> {
    fn set_targets(&self, paths: &[String]) -> CommandResult {
        let resolved = self.prepare_targets(paths)?;
        let unique = Self::dedupe_preserving_order(resolved.iter().map(|(abs, _)| abs.clone()));
        self.config_repo.set_marked_targets(&unique)?;

        let unique_lookup: HashSet<String> = unique.iter().cloned().collect();
        let mut printed = HashSet::new();
        for (abs_path, info) in resolved {
            if unique_lookup.contains(&abs_path) && printed.insert(abs_path.clone()) {
                let target_type = Self::target_type_from_info(&info);
                println!(
                    "✅ : {} {} marked.",
                    self.ui_service.get_icon_for_target_type(target_type),
                    self.ui_service.get_colorized_text(&abs_path, true)
                );
            }
        }

        self.print_list_summary(unique.len())
    }

    fn add_targets(&self, paths: &[String]) -> CommandResult {
        let resolved = self.prepare_targets(paths)?;
        let existing = self.config_repo.get_marked_targets()?;
        let mut seen_all: HashSet<String> = HashSet::new();
        let mut current = Vec::with_capacity(existing.len() + resolved.len());
        for value in existing {
            if seen_all.insert(value.clone()) {
                current.push(value);
            }
        }

        let mut added = Vec::new();
        for (abs_path, info) in resolved {
            if seen_all.insert(abs_path.clone()) {
                current.push(abs_path.clone());
                added.push((abs_path, info));
            }
        }
        self.config_repo.set_marked_targets(&current)?;

        if added.is_empty() {
            println!(
                "{} : All provided paths were already marked.",
                self.ui_service.get_icon_information()
            );
        } else {
            for (abs_path, info) in added {
                let target_type = Self::target_type_from_info(&info);
                println!(
                    "✅ : {} {} added.",
                    self.ui_service.get_icon_for_target_type(target_type),
                    self.ui_service.get_colorized_text(&abs_path, true)
                );
            }
        }

        self.print_list_summary(current.len())
    }

    fn list_targets(&self) -> CommandResult {
        let targets = self.config_repo.get_marked_targets()?;
        if targets.is_empty() {
            println!(
                "{} : No targets marked.",
                self.ui_service.get_icon_information()
            );
            return Ok(());
        }

        println!(
            "{} : Marked targets:",
            self.ui_service.get_icon_information()
        );
        for target in targets {
            let icon = self
                .ui_service
                .get_icon_for_target_type(self.fs_service.get_file_type(&target));
            println!("{} {}", icon, target);
        }
        Ok(())
    }

    fn reset_targets(&self) -> CommandResult {
        self.config_repo.reset_marks()?;
        println!("✅ : Marked targets cleared.");
        self.print_list_summary(0)?;
        Ok(())
    }

    fn prepare_targets(&self, paths: &[String]) -> FugaResult<Vec<(String, FileInfo)>> {
        let mut resolved = Vec::with_capacity(paths.len());
        for path in paths {
            let abs_path = self.fs_service.get_abs_path(path)?;
            let info = self.fs_service.get_file_info(&abs_path)?;
            if !info.exists {
                return Err(FugaError::FileNotFound(abs_path));
            }
            resolved.push((abs_path, info));
        }
        Ok(resolved)
    }

    fn dedupe_preserving_order<I>(values: I) -> Vec<String>
    where
        I: IntoIterator<Item = String>,
    {
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for value in values {
            if seen.insert(value.clone()) {
                result.push(value);
            }
        }
        result
    }

    fn target_type_from_info(info: &FileInfo) -> TargetType {
        if info.is_file {
            TargetType::File
        } else {
            TargetType::Dir
        }
    }

    fn print_list_summary(&self, count: usize) -> CommandResult {
        println!(
            "{} : Mark list now tracks {} target(s).",
            self.ui_service.get_icon_information(),
            count
        );
        Ok(())
    }
}
