//! Confirmation request utilities for CLI
//!
//! This module contains utilities for handling confirmation prompts.

use std::io::{self, Write};

/// Helper to handle confirmation prompts
#[cfg(feature = "cli")]
pub struct ConfirmationRequest {
    pub prompt: String,
    pub default: bool,
}

#[cfg(feature = "cli")]
impl Default for ConfirmationRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfirmationRequest {
    /// Create a new confirmation request
    pub fn new() -> Self {
        Self {
            prompt: "Are you sure?".to_string(),
            default: true,
        }
    }

    /// Set the prompt text
    pub fn with_prompt(mut self, prompt: &str) -> Self {
        self.prompt = prompt.to_string();
        self
    }

    /// Set the default answer
    pub fn with_default(mut self, default: bool) -> Self {
        self.default = default;
        self
    }

    /// Ask for confirmation and return the result
    pub fn ask(&self) -> bool {
        let default_text = if self.default { "Y/n" } else { "y/N" };
        let prompt = format!("{} [{}]: ", self.prompt, default_text);

        loop {
            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                return self.default;
            }

            let input = input.trim().to_lowercase();
            if input.is_empty() {
                return self.default;
            }

            if input == "y" || input == "yes" {
                return true;
            }

            if input == "n" || input == "no" {
                return false;
            }
        }
    }

    /// Ask for confirmation and return the result, with custom yes/no values
    pub fn ask_with_values(&self, yes_values: &[&str], no_values: &[&str]) -> bool {
        let default_text = if self.default {
            format!("{}/{}", yes_values[0], no_values[0].to_lowercase())
        } else {
            format!("{}/{}", yes_values[0].to_lowercase(), no_values[0])
        };

        let prompt = format!("{} [{}]: ", self.prompt, default_text);

        loop {
            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                return self.default;
            }

            let input = input.trim().to_lowercase();
            if input.is_empty() {
                return self.default;
            }

            for yes_value in yes_values {
                if input == yes_value.to_lowercase() {
                    return true;
                }
            }

            for no_value in no_values {
                if input == no_value.to_lowercase() {
                    return false;
                }
            }

            println!(
                "Please enter one of: {}, {}",
                yes_values.join(", "),
                no_values.join(", ")
            );
        }
    }
}
