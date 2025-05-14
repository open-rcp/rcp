//! CLI utility module
//!
//! This module provides utility functions for CLI operations.

#[cfg(feature = "cli")]
use crate::cli::error::CliError;
#[cfg(feature = "cli")]
use anyhow::Result;
#[cfg(feature = "cli")]
use colored::Colorize;

// Submodules
#[cfg(feature = "cli")]
pub mod confirmation;

/// CLI output formatting utilities
#[cfg(feature = "cli")]
pub struct OutputFormatter {
    pub color_enabled: bool,
    pub json_output: bool,
    pub quiet: bool,
}

#[cfg(feature = "cli")]
impl OutputFormatter {
    /// Create a new formatter with default settings
    pub fn new(json_output: bool, color_enabled: bool, quiet: bool) -> Self {
        Self {
            color_enabled,
            json_output,
            quiet,
        }
    }

    /// Print success message
    pub fn success(&self, message: &str) {
        if self.quiet {
            return;
        }

        if self.json_output {
            println!("{{\"status\":\"success\",\"message\":\"{}\"}}", message);
            return;
        }

        if self.color_enabled {
            println!("{} {}", "SUCCESS:".green().bold(), message);
        } else {
            println!("SUCCESS: {}", message);
        }
    }

    /// Print error message
    pub fn error(&self, message: &str) {
        if self.quiet {
            return;
        }

        if self.json_output {
            println!("{{\"status\":\"error\",\"message\":\"{}\"}}", message);
            return;
        }

        if self.color_enabled {
            println!("{} {}", "ERROR:".red().bold(), message);
        } else {
            println!("ERROR: {}", message);
        }
    }

    /// Print warning message
    pub fn warning(&self, message: &str) {
        if self.quiet {
            return;
        }

        if self.json_output {
            println!("{{\"status\":\"warning\",\"message\":\"{}\"}}", message);
            return;
        }

        if self.color_enabled {
            println!("{} {}", "WARNING:".yellow().bold(), message);
        } else {
            println!("WARNING: {}", message);
        }
    }

    /// Print info message
    pub fn info(&self, message: &str) {
        if self.quiet {
            return;
        }

        if self.json_output {
            println!("{{\"status\":\"info\",\"message\":\"{}\"}}", message);
            return;
        }

        if self.color_enabled {
            println!("{} {}", "INFO:".blue().bold(), message);
        } else {
            println!("INFO: {}", message);
        }
    }

    /// Print output success message
    pub fn output_success(&self, message: &str) {
        self.success(message);
    }

    /// Output item with header
    pub fn output_item<T: serde::Serialize + std::fmt::Display>(
        &self,
        item: &T,
        header: &str,
    ) -> Result<()> {
        if self.quiet {
            return Ok(());
        }

        if self.json_output {
            let json = serde_json::to_string_pretty(item)?;
            println!("{}", json);
            return Ok(());
        }

        if !header.is_empty() {
            if self.color_enabled {
                println!("{}", header.blue().bold());
                println!("{}", "=".repeat(header.len()).blue());
            } else {
                println!("{}", header);
                println!("{}", "=".repeat(header.len()));
            }
        }

        println!("{}", item);
        Ok(())
    }

    /// Output list of items with header
    pub fn output_list<T: serde::Serialize + std::fmt::Display>(
        &self,
        items: &[T],
        header: &str,
        empty_message: &str,
    ) -> Result<()> {
        if self.quiet {
            return Ok(());
        }

        if self.json_output {
            let json = serde_json::to_string_pretty(items)?;
            println!("{}", json);
            return Ok(());
        }

        if items.is_empty() {
            if !empty_message.is_empty() {
                if self.color_enabled {
                    println!("{}", empty_message.yellow());
                } else {
                    println!("{}", empty_message);
                }
            }
            return Ok(());
        }

        if !header.is_empty() {
            if self.color_enabled {
                println!("{}", header.blue().bold());
                println!("{}", "=".repeat(header.len()).blue());
            } else {
                println!("{}", header);
                println!("{}", "=".repeat(header.len()));
            }
        }

        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                println!();
            }
            println!("{}", item);
        }

        Ok(())
    }

    /// Print data as JSON
    pub fn json<T: serde::Serialize>(&self, data: T) -> Result<(), CliError> {
        if self.quiet {
            return Ok(());
        }

        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| CliError::SerializationError(e.to_string()))?;

        println!("{}", json);

        Ok(())
    }

    /// Print data as a table
    pub fn table<F>(&self, headers: Vec<&str>, row_fn: F)
    where
        F: FnOnce(&mut TableBuilder),
    {
        if self.quiet {
            return;
        }

        let mut builder = TableBuilder::new(headers);
        row_fn(&mut builder);

        if self.json_output {
            if let Ok(json) = serde_json::to_string_pretty(&builder.to_json()) {
                println!("{}", json);
            }
            return;
        }

        builder.print(self.color_enabled);
    }
}

/// Table builder for formatting tabular data
#[cfg(feature = "cli")]
pub struct TableBuilder {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[cfg(feature = "cli")]
impl TableBuilder {
    /// Create a new table builder
    pub fn new(headers: Vec<&str>) -> Self {
        Self {
            headers: headers.into_iter().map(|s| s.to_string()).collect(),
            rows: Vec::new(),
        }
    }

    /// Add a row to the table
    pub fn add_row(&mut self, values: Vec<&str>) {
        let row = values.into_iter().map(|s| s.to_string()).collect();
        self.rows.push(row);
    }

    /// Convert the table to a JSON object
    pub fn to_json(&self) -> serde_json::Value {
        let mut result = Vec::new();

        for row in &self.rows {
            let mut obj = serde_json::Map::new();

            for (i, header) in self.headers.iter().enumerate() {
                if i < row.len() {
                    obj.insert(header.clone(), serde_json::Value::String(row[i].clone()));
                }
            }

            result.push(serde_json::Value::Object(obj));
        }

        serde_json::Value::Array(result)
    }

    /// Print the table
    pub fn print(&self, color_enabled: bool) {
        if self.rows.is_empty() {
            println!("No data available.");
            return;
        }

        // Calculate column widths
        let mut widths = vec![0; self.headers.len()];

        for (i, header) in self.headers.iter().enumerate() {
            widths[i] = header.len();
        }

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }

        // Print header
        let header_row = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:width$}", h, width = widths[i]))
            .collect::<Vec<_>>()
            .join(" | ");

        if color_enabled {
            println!("{}", header_row.bold());
        } else {
            println!("{}", header_row);
        }

        // Print separator
        let separator = widths
            .iter()
            .map(|w| "-".repeat(*w))
            .collect::<Vec<_>>()
            .join("-+-");

        println!("{}", separator);

        // Print rows
        for row in &self.rows {
            let row_str = row
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    if i < widths.len() {
                        format!("{:width$}", cell, width = widths[i])
                    } else {
                        cell.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join(" | ");

            println!("{}", row_str);
        }
    }
}

/// Load configuration from file
#[cfg(feature = "cli")]
pub fn load_config(
    config_path: Option<std::path::PathBuf>,
) -> Result<crate::cli::config::CliConfig> {
    use crate::cli::config::CliConfig;
    use std::fs;

    // Determine the configuration file path
    let path = if let Some(path) = config_path {
        path
    } else {
        // Try to find the default config file location
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
        let config_dir = home.join(".config").join("rcp");

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)
                .map_err(|e| anyhow::anyhow!("Failed to create config directory: {}", e))?;
        }

        config_dir.join("config.toml")
    };

    // Try to read the config file
    if path.exists() {
        let content = fs::read_to_string(&path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;

        let config: CliConfig = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?;

        Ok(config)
    } else {
        // Return default config if file doesn't exist
        Ok(CliConfig::default())
    }
}

/// Save configuration to file
#[cfg(feature = "cli")]
pub fn save_config(
    config: &crate::cli::config::CliConfig,
    config_path: std::path::PathBuf,
) -> Result<()> {
    use std::fs;
    use std::io::Write;

    // Ensure the parent directory exists
    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| anyhow::anyhow!("Failed to create config directory: {}", e))?;
        }
    }

    // Serialize config to TOML
    let content = toml::to_string_pretty(config)
        .map_err(|e| anyhow::anyhow!("Failed to serialize config: {}", e))?;

    // Write config to file
    let mut file = fs::File::create(&config_path)
        .map_err(|e| anyhow::anyhow!("Failed to create config file: {}", e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to write config file: {}", e))?;

    Ok(())
}

/// Format a duration in seconds to a human-readable format
#[cfg(feature = "cli")]
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{} seconds", seconds);
    }

    let minutes = seconds / 60;
    if minutes < 60 {
        return format!("{} minutes", minutes);
    }

    let hours = minutes / 60;
    if hours < 24 {
        return format!("{} hours", hours);
    }

    let days = hours / 24;
    format!("{} days", days)
}
