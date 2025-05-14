//! Command module for shell completion generation
//!
//! This module contains the command handlers for generating shell completions.
//! Ported from rcp-cli component as part of CLI unification.

#[cfg(feature = "cli")]
use anyhow::Result;
#[cfg(feature = "cli")]
use clap::CommandFactory;
#[cfg(feature = "cli")]
use clap_complete::{generate, generate_to};
#[cfg(feature = "cli")]
use std::io;
#[cfg(feature = "cli")]
use std::path::Path;

#[cfg(feature = "cli")]
use crate::Cli;

/// Handle completions command
#[cfg(feature = "cli")]
pub fn handle_completions_command(shell: clap_complete::Shell, dir: Option<&Path>) -> Result<()> {
    // Get command from clap
    use crate::Cli;
    use clap::CommandFactory;
    let mut cmd = Cli::command();

    if let Some(dir) = dir {
        generate_to(shell, &mut cmd, "rcpd", dir)?;
        println!(
            "Completions for shell '{}' written to directory: {}",
            shell,
            dir.display()
        );
    } else {
        generate(shell, &mut cmd, "rcpd", &mut io::stdout());
    }

    Ok(())
}

/// Auto-detect current shell and generate completions
#[cfg(feature = "cli")]
pub fn handle_auto_completions(dir: Option<&Path>) -> Result<()> {
    // Try to detect shell from environment
    let shell = if let Ok(shell_env) = std::env::var("SHELL") {
        if shell_env.contains("bash") {
            clap_complete::Shell::Bash
        } else if shell_env.contains("zsh") {
            clap_complete::Shell::Zsh
        } else if shell_env.contains("fish") {
            clap_complete::Shell::Fish
        } else {
            // Default to bash if we can't identify the shell
            clap_complete::Shell::Bash
        }
    } else {
        // Default to bash if SHELL is not set
        clap_complete::Shell::Bash
    };

    handle_completions_command(shell, dir)
}
