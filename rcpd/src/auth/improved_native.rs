//! Improvements for native authentication providers
//!
//! This module contains common utility functions and traits for improving
//! the native authentication providers.

use anyhow::{anyhow, Result};
use log::{debug, warn};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

/// Trait for enhanced group management
pub trait EnhancedGroupManagement {
    /// Get all groups a user belongs to with improved error handling
    fn get_user_groups_enhanced(
        &self,
        username: &str,
        cache: &mut HashMap<String, Vec<String>>,
    ) -> Result<Vec<String>>;

    /// Map OS groups to RCP permissions with fallback
    fn map_permissions_enhanced(
        &self,
        groups: &[String],
        admin_groups: &[String],
        require_group: &Option<String>,
        permission_mappings: &HashMap<String, Vec<String>>,
    ) -> Vec<String>;
}

/// Implementation for macOS group management
pub fn get_macos_user_groups(
    username: &str,
    cache: &mut HashMap<String, Vec<String>>,
) -> Result<Vec<String>> {
    // Check cache first
    if let Some(groups) = cache.get(username) {
        debug!("Using cached groups for user: {}", username);
        return Ok(groups.clone());
    }

    debug!("Getting groups for user: {}", username);

    // Use dscl to get all groups
    let output = Command::new("dscl")
        .args(&[".", "-list", "/Groups", "GroupMembership"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to list groups: {:?}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut groups = Vec::new();

    for line in output_str.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 1 {
            let group_name = parts[0];
            let members = parts[1..].iter().cloned().collect::<Vec<&str>>();

            if members.contains(&username) {
                groups.push(group_name.to_string());
            }
        }
    }

    // Check for empty results - this could indicate an error
    if groups.is_empty() {
        warn!("No groups found for user: {}", username);

        // As a fallback, try getting primary group
        let primary_output = Command::new("id").args(&["-gn", username]).output()?;

        if primary_output.status.success() {
            let primary_group = String::from_utf8_lossy(&primary_output.stdout)
                .trim()
                .to_string();
            if !primary_group.is_empty() {
                debug!("Adding primary group for {}: {}", username, primary_group);
                groups.push(primary_group);
            }
        }
    }

    debug!("Found groups for {}: {:?}", username, groups);

    // Update cache
    cache.insert(username.to_string(), groups.clone());

    Ok(groups)
}

/// Implementation for Linux group management
pub fn get_linux_user_groups(
    username: &str,
    cache: &mut HashMap<String, Vec<String>>,
) -> Result<Vec<String>> {
    // Check cache first
    if let Some(groups) = cache.get(username) {
        debug!("Using cached groups for user: {}", username);
        return Ok(groups.clone());
    }

    debug!("Getting groups for user: {}", username);

    // Use the 'groups' command
    let output = Command::new("groups").arg(username).output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to list groups: {:?}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let groups_str = output_str.trim();

    // Format is typically: "username : group1 group2 group3"
    let parts: Vec<&str> = groups_str.split(':').collect();
    let groups: Vec<String> = if parts.len() > 1 {
        parts[1]
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    } else {
        // Some systems might just return the groups without the username prefix
        groups_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    };

    debug!("Found groups for {}: {:?}", username, groups);

    // Update cache
    cache.insert(username.to_string(), groups.clone());

    Ok(groups)
}

/// Implementation for Windows group management
pub fn get_windows_user_groups(
    username: &str,
    cache: &mut HashMap<String, Vec<String>>,
) -> Result<Vec<String>> {
    // Check cache first
    if let Some(groups) = cache.get(username) {
        debug!("Using cached groups for user: {}", username);
        return Ok(groups.clone());
    }

    debug!("Getting groups for user: {}", username);

    // Use PowerShell to get user groups
    let ps_command = format!(
        "Get-LocalGroupMember -Member {} | Select-Object -ExpandProperty Group | Select-Object -ExpandProperty Name",
        username
    );

    let output = Command::new("powershell")
        .args(&["-Command", &ps_command])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to list groups: {:?}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let groups: Vec<String> = output_str
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    debug!("Found groups for {}: {:?}", username, groups);

    // Update cache
    cache.insert(username.to_string(), groups.clone());

    Ok(groups)
}

/// Implementation for generic Unix group management (FreeBSD, OpenBSD, NetBSD, etc.)
pub fn get_unix_user_groups(
    username: &str,
    cache: &mut HashMap<String, Vec<String>>,
) -> Result<Vec<String>> {
    // Check cache first
    if let Some(groups) = cache.get(username) {
        debug!("Using cached groups for user: {}", username);
        return Ok(groups.clone());
    }

    debug!("Getting groups for user: {}", username);

    let mut groups = Vec::new();
    let mut found_groups = false; // Renamed from 'success' for clarity

    // First approach: Use the 'groups' command which is available on most Unix systems
    let output = Command::new("groups").arg(username).output();

    if let Ok(output) = output {
        if output.status.success() {
            found_groups = true; // Mark that we found groups
            let output_str = String::from_utf8_lossy(&output.stdout);

            // Parse output which might be in one of these formats:
            // 1. "username : group1 group2 group3" (Linux style)
            // 2. "group1 group2 group3" (FreeBSD style)
            let parts: Vec<&str> = output_str.split(':').collect();
            let groups_str = if parts.len() > 1 {
                parts[1].trim()
            } else {
                output_str.trim()
            };

            for group in groups_str.split_whitespace() {
                groups.push(group.to_string());
            }

            debug!("Found groups using 'groups' command: {:?}", groups);
        } else {
            warn!(
                "'groups' command failed: {:?}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    // If the groups command failed or returned empty results, try alternate methods
    if !found_groups || groups.is_empty() {
        warn!(
            "No groups found for user: {} using 'groups' command, trying alternate methods",
            username
        );

        // Fallback 1: Try using getent
        let getent_output = Command::new("getent").args(&["group"]).output();

        if let Ok(output) = getent_output {
            if output.status.success() {
                debug!("Using 'getent group' to find memberships");
                let getent_str = String::from_utf8_lossy(&output.stdout);
                for line in getent_str.lines() {
                    let group_parts: Vec<&str> = line.split(':').collect();
                    if group_parts.len() >= 4 {
                        let group_name = group_parts[0];
                        let members = group_parts[3];

                        if members.split(',').any(|m| m.trim() == username) {
                            debug!(
                                "Found group {} for user {} using getent",
                                group_name, username
                            );
                            groups.push(group_name.to_string());
                        }
                    }
                }

                if !groups.is_empty() {
                    found_groups = true;
                }
            } else {
                debug!("'getent group' command failed or not available");
            }
        }

        // Fallback 2: Try 'id -G -n' command (works on most BSD systems and some Unix variants)
        if !found_groups || groups.is_empty() {
            debug!("Trying 'id -G -n' command");
            let id_output = Command::new("id").args(&["-G", "-n", username]).output();

            if let Ok(output) = id_output {
                if output.status.success() {
                    let id_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !id_str.is_empty() {
                        // Split on spaces or tabs (different Unix variants use different separators)
                        for group in id_str.split(|c: char| c.is_whitespace()) {
                            if !group.is_empty() && !groups.contains(&group.to_string()) {
                                groups.push(group.to_string());
                            }
                        }

                        debug!("Found groups using 'id' command: {:?}", groups);
                        // We only want to track if we found groups successfully for debugging
                        // and don't need to use this value later
                        let _ = groups.len() > 0;
                    }
                }
            }
        }

        // Fallback 3: Check /etc/group directly (works on most Unix systems)
        if !found_groups || groups.is_empty() {
            debug!("Trying to parse /etc/group file directly");
            if let Ok(group_contents) = std::fs::read_to_string("/etc/group") {
                for line in group_contents.lines() {
                    let group_parts: Vec<&str> = line.split(':').collect();
                    if group_parts.len() >= 4 {
                        let group_name = group_parts[0];
                        let members = group_parts[3];

                        if members.split(',').any(|m| m.trim() == username) {
                            debug!("Found group {} in /etc/group", group_name);
                            if !groups.contains(&group_name.to_string()) {
                                groups.push(group_name.to_string());
                            }
                        }
                    }
                }

                if !groups.is_empty() {
                    found_groups = true;
                }
            } else {
                debug!("Could not read /etc/group file");
            }
        }
    }

    debug!("Found groups for {}: {:?}", username, groups);

    // Update cache
    cache.insert(username.to_string(), groups.clone());

    Ok(groups)
}

/// Detect the Unix variant for better platform-specific behavior
pub fn detect_unix_platform() -> String {
    // First try uname -s
    if let Ok(output) = Command::new("uname").arg("-s").output() {
        if output.status.success() {
            let os_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            match os_name.as_str() {
                "FreeBSD" | "OpenBSD" | "NetBSD" | "DragonFly" => return os_name,
                "Linux" => return "Linux".to_string(),
                "Darwin" => return "macOS".to_string(),
                "SunOS" => return "Solaris".to_string(),
                _ => {
                    // Return as-is for other Unix variants
                    if !os_name.is_empty() {
                        return os_name;
                    }
                }
            }
        }
    }

    // Fallback detection method
    if Path::new("/etc/redhat-release").exists() {
        return "RedHat-based".to_string();
    } else if Path::new("/etc/debian_version").exists() {
        return "Debian-based".to_string();
    } else if Path::new("/etc/alpine-release").exists() {
        return "Alpine".to_string();
    } else if Path::new("/etc/freebsd-update.conf").exists() {
        return "FreeBSD".to_string();
    } else if Path::new("/etc/rc.conf").exists() && Path::new("/etc/pwd.db").exists() {
        return "BSD-variant".to_string();
    }

    // Generic fallback
    "Unknown-Unix".to_string()
}

/// Get common admin groups for the detected platform
pub fn get_platform_admin_groups() -> Vec<String> {
    let platform = detect_unix_platform();
    let mut admin_groups = vec![
        "wheel".to_string(), // Common across many Unix systems
    ];

    match platform.as_str() {
        "Linux" => {
            admin_groups.push("sudo".to_string());
            admin_groups.push("admin".to_string());
            admin_groups.push("adm".to_string());
        }
        "macOS" => {
            admin_groups.push("admin".to_string());
            admin_groups.push("staff".to_string());
        }
        "FreeBSD" | "OpenBSD" | "NetBSD" | "DragonFly" | "BSD-variant" => {
            admin_groups.push("operator".to_string());
        }
        "Solaris" => {
            admin_groups.push("sys".to_string());
            admin_groups.push("root".to_string());
        }
        _ => {
            // Add some common ones for unknown Unix variants
            admin_groups.push("admin".to_string());
            admin_groups.push("root".to_string());
            admin_groups.push("system".to_string());
        }
    }

    admin_groups
}

/// Common implementation for mapping permissions
pub fn map_permissions_common(
    groups: &[String],
    admin_groups: &[String],
    require_group: &Option<String>,
    permission_mappings: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut permissions = Vec::new();

    // Check for admin groups
    let is_admin = groups.iter().any(|g| admin_groups.contains(g));
    if is_admin {
        permissions.push("admin:*".to_string());
        permissions.push("connect:*".to_string());
        permissions.push("app:*".to_string());
        return permissions;
    }

    // Apply custom permission mappings
    for group in groups {
        if let Some(group_permissions) = permission_mappings.get(group) {
            for permission in group_permissions {
                if !permissions.contains(permission) {
                    permissions.push(permission.clone());
                }
            }
        }
    }

    // If no permissions were assigned through mappings but the user is in the required group,
    // grant basic connection permission
    if permissions.is_empty()
        && require_group
            .as_ref()
            .map_or(false, |required_group| groups.contains(&required_group))
    {
        permissions.push("connect:basic".to_string());
    }

    permissions
}
