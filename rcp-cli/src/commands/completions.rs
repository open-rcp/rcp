use clap::Command;
#[cfg(test)]
use clap_complete::generate_to;
use clap_complete::{generate, Shell};
use std::io;
#[cfg(test)]
use std::path::Path;

/// Helper function for tests to check if this module is properly loaded
#[cfg(test)]
pub fn is_module_loaded() -> bool {
    true
}

pub fn handle_completions_command(cmd: &mut Command, shell: Shell) -> Result<(), io::Error> {
    generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout());
    Ok(())
}

/// Generate completions to a file, useful for testing
#[cfg(test)]
pub fn generate_completions(
    shell_name: &str,
    dir: Option<impl AsRef<Path>>,
) -> Result<(), io::Error> {
    let mut cmd = Command::new("rcp-cli");
    let shell = match shell_name {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" => Shell::PowerShell,
        _ => Shell::Bash, // Default to bash
    };

    if let Some(dir) = dir {
        generate_to(shell, &mut cmd, "rcp-cli", dir.as_ref())?;
    } else {
        generate(shell, &mut cmd, "rcp-cli", &mut io::stdout());
    }

    Ok(())
}
