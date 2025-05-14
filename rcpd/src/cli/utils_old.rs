//! CLI utility module
//!
//! This module provides utility functions for CLI operations.

#[cfg(feature = "cli")]
use crate::cli::error::CliError;
#[cfg(feature = "cli")]
use colored::Colorize;
#[cfg(feature = "cli")]
use anyhow::Result;
#[cfg(feature = "cli")]
use std::path::PathBuf;

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
        if self.quiet && !self.json_output {
            return;
        }

        if self.json_output {
            println!("{{\"status\":\"error\",\"message\":\"{}\"}}", message);
            return;
        }

        if self.color_enabled {
            eprintln!("{} {}", "ERROR:".red().bold(), message);
        } else {
            eprintln!("ERROR: {}", message);
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

    /// Print data as JSON
    pub fn json<T: serde::Serialize>(&self, data: &T) -> Result<(), CliError> {
        if self.quiet {
            return Ok(());
        }

        let json = serde_json::to_string_pretty(data)?;
        println!("{}", json);
        Ok(())
    }

    /// Print table data
    pub fn table<F>(&self, headers: &[&str], rows: &[Vec<String>], colorize_fn: Option<F>) 
    where
        F: Fn(&str, usize, usize) -> String,
    {
        if self.quiet {
            return;
        }

        if self.json_output {
            let mut table_data = Vec::new();
            for row in rows {
                let mut row_data = std::collections::HashMap::new();
                for (i, header) in headers.iter().enumerate() {
                    if i < row.len() {
                        row_data.insert(*header, row[i].clone());
                    }
                }
                table_data.push(row_data);
            }
            
            if let Ok(json) = serde_json::to_string_pretty(&table_data) {
                println!("{}", json);
            }
            return;
        }

        // Calculate column widths
        let mut widths = vec![0; headers.len()];
        
        // Check header lengths
        for (i, header) in headers.iter().enumerate() {
            widths[i] = widths[i].max(header.len());
        }
        
        // Check data lengths
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }
        
        // Print headers
        let mut header_line = String::new();
        for (i, header) in headers.iter().enumerate() {
            let formatted_header = if self.color_enabled {
                format!("{:width$}", header, width = widths[i]).bold().to_string()
            } else {
                format!("{:width$}", header, width = widths[i])
            };
            
            header_line.push_str(&formatted_header);
            if i < headers.len() - 1 {
                header_line.push_str("  ");
            }
        }
        println!("{}", header_line);
        
        // Print separator
        let separator: String = widths.iter()
            .map(|&width| "-".repeat(width))
            .collect::<Vec<String>>()
            .join("  ");
        println!("{}", separator);
        
        // Print rows
        for (row_idx, row) in rows.iter().enumerate() {
            let mut line = String::new();
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx >= widths.len() {
                    continue;
                }
                
                let formatted_cell = if self.color_enabled && colorize_fn.is_some() {
                    colorize_fn.as_ref().unwrap()(cell, row_idx, col_idx)
                } else {
                    format!("{:width$}", cell, width = widths[col_idx])
                };
                
                line.push_str(&formatted_cell);
                if col_idx < row.len() - 1 && col_idx < widths.len() - 1 {
                    line.push_str("  ");
                }
            }
            println!("{}", line);
        }
    }
}

/// Determine if colors should be enabled
#[cfg(feature = "cli")]
pub fn should_enable_colors(force_color: Option<bool>) -> bool {
    if let Some(force) = force_color {
        return force;
    }
    
    // Check for NO_COLOR environment variable
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }
    
    // Check for CLICOLOR/CLICOLOR_FORCE
    if std::env::var("CLICOLOR_FORCE").is_ok() {
        return true;
    }
    
    if let Ok(val) = std::env::var("CLICOLOR") {
        if val == "0" {
            return false;
        }
    }
    
    // Check if stdout is a terminal
    atty::is(atty::Stream::Stdout)
}

/// Load CLI configuration
#[cfg(feature = "cli")]
pub fn load_config(config_path: Option<PathBuf>) -> Result<crate::cli::CliConfig> {
    let default_config = crate::cli::CliConfig::default();
    
    // If no config path specified, look in default locations
    let config_path = match config_path {
        Some(path) => path,
        None => {
            if let Some(mut config_dir) = dirs::config_dir() {
                config_dir.push("rcp");
                std::fs::create_dir_all(&config_dir)?;
                config_dir.push("cli-config.toml");
                config_dir
            } else {
                return Ok(default_config);
            }
        }
    };
    
    // If config file doesn't exist, return default
    if !config_path.exists() {
        return Ok(default_config);
    }
    
    // Read and parse config file
    let config_str = std::fs::read_to_string(config_path)?;
    let config: crate::cli::CliConfig = toml::from_str(&config_str)
        .map_err(|e| CliError::ConfigError(e.to_string()))?;
    
    Ok(config)
}

/// Save CLI configuration
#[cfg(feature = "cli")]
pub fn save_config(config: &crate::cli::CliConfig, config_path: Option<PathBuf>) -> Result<()> {
    let config_path = match config_path {
        Some(path) => path,
        None => {
            let mut config_dir = dirs::config_dir()
                .ok_or_else(|| CliError::ConfigError("Could not determine config directory".to_string()))?;
            config_dir.push("rcp");
            std::fs::create_dir_all(&config_dir)?;
            config_dir.push("cli-config.toml");
            config_dir
        }
    };
    
    let config_str = toml::to_string_pretty(config)
        .map_err(|e| CliError::ConfigError(e.to_string()))?;
    
    std::fs::write(config_path, config_str)?;
    
    Ok(())
}
