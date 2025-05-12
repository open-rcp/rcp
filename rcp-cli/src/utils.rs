use anyhow::Result;
use chrono::Duration;
use colored::Colorize;
use humantime::parse_duration;
use std::io::{self, Write};

/// Format duration from seconds to human readable string
pub fn format_duration(seconds: u64) -> String {
    let duration = Duration::seconds(seconds as i64);

    if duration.num_days() > 0 {
        format!(
            "{}d {}h {}m {}s",
            duration.num_days(),
            duration.num_hours() % 24,
            duration.num_minutes() % 60,
            duration.num_seconds() % 60
        )
    } else if duration.num_hours() > 0 {
        format!(
            "{}h {}m {}s",
            duration.num_hours(),
            duration.num_minutes() % 60,
            duration.num_seconds() % 60
        )
    } else if duration.num_minutes() > 0 {
        format!(
            "{}m {}s",
            duration.num_minutes(),
            duration.num_seconds() % 60
        )
    } else {
        format!("{}s", duration.num_seconds())
    }
}

/// Parse a duration string like "1h", "30m", "1d", etc.
#[allow(dead_code)]
pub fn parse_time_string(time_str: &str) -> Result<Duration> {
    let std_duration = parse_duration(time_str)?;
    let seconds = std_duration.as_secs();
    Ok(Duration::seconds(seconds as i64))
}

/// Format status with appropriate color
pub fn format_status(status: &str) -> colored::ColoredString {
    match status.to_lowercase().as_str() {
        "running" | "active" | "online" | "connected" => status.green(),
        "stopped" | "inactive" | "offline" | "disconnected" => status.red(),
        "starting" | "stopping" | "restarting" => status.yellow(),
        "warning" => status.yellow(),
        "error" | "failed" => status.red(),
        _ => status.normal(),
    }
}

/// Prompt user for confirmation
pub fn confirm(prompt: &str, default: bool) -> bool {
    let mut input = String::new();

    let default_str = if default { "Y/n" } else { "y/N" };
    print!("{} [{}]: ", prompt, default_str);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();
    if input.is_empty() {
        default
    } else { input == "y" || input == "yes" }
}

/// Prompt user for input
#[allow(dead_code)]
pub fn prompt(prompt: &str, default: Option<&str>) -> Result<String> {
    let mut input = String::new();

    match default {
        Some(default_value) => {
            print!("{} [{}]: ", prompt, default_value);
        }
        None => {
            print!("{}: ", prompt);
        }
    }

    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    let input = input.trim();
    if input.is_empty() {
        match default {
            Some(default_value) => Ok(default_value.to_string()),
            None => Err(anyhow::anyhow!("Input required")),
        }
    } else {
        Ok(input.to_string())
    }
}

/// Print a table row with aligned columns
pub fn print_table_row(columns: &[&str], widths: &[usize]) {
    assert!(
        columns.len() == widths.len(),
        "Column and width counts must match"
    );

    for (idx, (col, width)) in columns.iter().zip(widths.iter()).enumerate() {
        if idx > 0 {
            print!(" ");
        }
        print!("{:width$}", col, width = width);
    }
    println!();
}

/// Print a section header
pub fn print_section(title: &str) {
    println!("\n{}", title.blue().bold());
    println!("{}", "=".repeat(title.len()).blue());
}
