use crate::commands::{Command, CommandResult};
use clap_complete::{generate, Generator, Shell};

/// Completion command for generating shell completion scripts
pub struct CompletionCommand {
    shell: Shell,
    cmd: clap::Command,
}

impl CompletionCommand {
    pub fn new(shell: Shell, cmd: clap::Command) -> Self {
        Self { shell, cmd }
    }

    fn print_completions<G: Generator>(&self, gen: G, cmd: &mut clap::Command) {
        generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
    }
}

impl Command for CompletionCommand {
    fn execute(&self) -> CommandResult {
        let mut cmd = self.cmd.clone();
        self.print_completions(self.shell, &mut cmd);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command as ClapCommand;
    use clap_complete::Shell;

    #[test]
    fn test_completion_command_creation() {
        let cmd = ClapCommand::new("test");
        let completion_cmd = CompletionCommand::new(Shell::Bash, cmd);

        assert_eq!(completion_cmd.shell, Shell::Bash);
        assert_eq!(completion_cmd.cmd.get_name(), "test");
    }

    #[test]
    fn test_completion_command_execute() {
        let cmd = ClapCommand::new("test")
            .about("Test application")
            .subcommand(ClapCommand::new("subtest"));
        let completion_cmd = CompletionCommand::new(Shell::Bash, cmd);

        // Should not panic and return Ok
        let result = completion_cmd.execute();
        assert!(result.is_ok());
    }

    #[test]
    fn test_different_shells() {
        let cmd = ClapCommand::new("test");

        let shells = vec![Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell];

        for shell in shells {
            let completion_cmd = CompletionCommand::new(shell, cmd.clone());
            assert_eq!(completion_cmd.shell, shell);

            // Should execute without error
            let result = completion_cmd.execute();
            assert!(result.is_ok());
        }
    }
}
