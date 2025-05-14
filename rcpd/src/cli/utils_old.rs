//! CLI utility module
//!
//! This module provides utility functions for CLI operations.

#[cfg(feature = "cli")]
use crate::cli::error::CliError;
#[cfg(feature = "cli")]
use colored::Colorize;
#[cfg(feature = "cli")]
use anyhow::Result;

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
                    obj.insert(
                        header.clone(),
                        serde_json::Value::String(row[i].clone()),
                    );
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
        let header_row = self.headers
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
