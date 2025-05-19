# Using Native OS Authentication with RCP

This guide explains how to configure and use the native OS user authentication feature in RCP.

## Overview

RCP can authenticate users against the host operating system instead of maintaining its own separate user database. This provides several benefits:

- **Single sign-on experience**: Users can use their OS credentials
- **Simplified user management**: Leverage existing OS tools and policies
- **Granular permission control**: Map OS groups to RCP permissions
- **Cross-platform support**: Works on Windows, macOS, Linux, FreeBSD, OpenBSD, NetBSD, and other Unix-like systems

## Enabling Native Authentication

To use native OS authentication, modify your `service.toml` configuration file:

```toml
[server.auth]
provider = "native"       # Use native OS authentication
required = true           # Authentication is required
psk = "customkey"         # Optional fallback for service accounts
fallback_to_internal = true  # Fall back to internal auth if needed

[server.auth.native]
allow_all_users = false   # Only specified users can connect
require_group = "rcp-users"  # OS group required for RCP access
permission_mapping = true # Map OS groups to RCP permissions
admin_groups = ["administrators", "wheel", "sudo"] # Admin groups

# Custom permission mappings
[server.auth.native.permission_mappings]
"administrators" = ["admin:*", "connect:*", "app:*"]
"rcp-users" = ["connect:*"]
"rcp-app-browsers" = ["app:safari", "app:chrome", "app:firefox"]
```

## Permission Mapping System

RCP's native authentication maps OS groups to RCP permissions. This creates a flexible security model:

1. **Group-Based Permissions**: Users inherit permissions based on their OS group memberships
2. **Wildcard Support**: Use `*` to grant access to all resources of a type (e.g., `app:*`)
3. **Permission Categories**:
   - `admin:*` - Administrative operations
   - `connect:*` - Connection permissions
   - `app:<name>` - Application-specific permissions

### Best Practices

1. Create dedicated groups for RCP permissions
2. Follow the principle of least privilege
3. Use specific application groups to control access
4. Audit group memberships regularly

## Platform-Specific Configuration

### Windows

On Windows, the system authenticates against local or domain accounts:

```toml
[server.auth.native]
require_group = "RCP Users"  # Windows group name
admin_groups = ["Administrators"]
```

To create the necessary groups:
```
net localgroup "RCP Users" /add
net localgroup "RCP-App-Safari" /add
net user bob /add
net localgroup "RCP Users" bob /add
```

### macOS

On macOS, the system authenticates against local Directory Services accounts:

```toml
[server.auth.native]
require_group = "staff"  # Most macOS users are in staff
admin_groups = ["admin", "wheel"]
```

To create application-specific groups:
```
sudo dscl . -create /Groups/rcp-app-safari
sudo dscl . -create /Groups/rcp-app-safari PrimaryGroupID 300
sudo dscl . -append /Groups/rcp-app-safari GroupMembership alice
```

### Linux

On Linux, the system authenticates against local accounts or directory services:

```toml
[server.auth.native]
require_group = "rcp-users"
admin_groups = ["sudo", "wheel", "admin"]
```

To create the necessary groups:
```
sudo groupadd rcp-users
sudo groupadd rcp-app-safari
sudo usermod -a -G rcp-users charlie
sudo usermod -a -G rcp-app-safari charlie
```

### FreeBSD, OpenBSD and other Unix-like Systems

On other Unix-like systems, the configuration is similar to Linux:

```toml
[server.auth.native]
require_group = "rcp-users"
admin_groups = ["wheel", "operator"]
```

To create the necessary groups (FreeBSD example):
```
pw groupadd rcp-users
pw groupadd rcp-app-safari
pw groupmod rcp-users -m dave
pw groupmod rcp-app-safari -m dave
```

For OpenBSD:
```
groupadd rcp-users
groupadd rcp-app-safari
usermod -G rcp-users dave
usermod -G rcp-app-safari dave
```

## Security Considerations

When using native authentication:

1. The RCP service requires appropriate system privileges to validate users
2. Consider using TLS to secure credentials in transit
3. Set restrictive file permissions on the configuration file
4. Review authentication logs regularly

## Troubleshooting

If you encounter issues with native authentication:

1. Check that the user exists on the OS
2. Verify the user is a member of the required group
3. Ensure the RCP service has permissions to read user/group information
4. Check system logs for authentication failures
5. Try with `fallback_to_internal = true` to see if internal auth works
