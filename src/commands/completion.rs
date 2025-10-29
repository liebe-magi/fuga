use crate::commands::{Command, CommandResult};
use clap_complete::{generate, Generator, Shell};
use std::io::{self, Write};

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
        #[cfg(test)]
        {
            let mut sink = io::sink();
            self.write_completions(gen, cmd, &mut sink);
        }

        #[cfg(not(test))]
        {
            let mut stdout = io::stdout();
            self.write_completions(gen, cmd, &mut stdout);
        }
    }

    fn write_completions<G: Generator, W: Write>(
        &self,
        gen: G,
        cmd: &mut clap::Command,
        writer: &mut W,
    ) {
        generate(gen, cmd, cmd.get_name().to_string(), writer);
    }

    #[cfg(test)]
    fn generate_completions_bytes(&self) -> Vec<u8> {
        let mut cmd = self.cmd.clone();
        let mut buffer = Vec::new();
        self.write_completions(self.shell, &mut cmd, &mut buffer);
        buffer
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
        let script = completion_cmd.generate_completions_bytes();
        assert!(!script.is_empty());

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
            let script = completion_cmd.generate_completions_bytes();
            assert!(!script.is_empty());
            let result = completion_cmd.execute();
            assert!(result.is_ok());
        }
    }
}
