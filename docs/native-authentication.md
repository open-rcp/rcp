# Native User Authentication for RCP

This document describes the native operating system user authentication implementation for the Rust/Remote Control Protocol (RCP).

## Overview

Native user authentication allows RCP to leverage the host operating system's existing user accounts instead of maintaining a separate user database. This provides several benefits:

- Single sign-on experience for users
- Simplified user management (leveraging existing OS tools)
- Consistent security policies across applications
- Integration with enterprise directory services (Active Directory, LDAP, etc.)

## Authentication Methods

The native authentication provider supports multiple authentication mechanisms:

### 1. Pre-Shared Key (PSK) with OS User Validation

This method combines the simplicity of PSK authentication with OS user validation. The client provides:
- A username (matching an OS account)
- The pre-shared key
- Optional system credentials (depending on configuration)

### 2. Public Key Authentication

Similar to SSH, this method uses asymmetric cryptography where:
- User public keys are registered with the OS
- Client authentication uses the corresponding private key
- Additional OS-level permissions are validated after key verification

### 3. Direct OS Authentication

This method directly validates OS credentials:
- On Windows: NTLM, Kerberos, or Windows Hello
- On macOS: Directory Services or Touch ID
- On Linux: PAM modules or SSSD

## Configuration

To enable native user authentication, modify the RCP service configuration:

```toml
[server.auth]
provider = "native"       # Use native OS authentication provider
required = true           # Authentication is required
psk = "customkey"         # Optional fallback for service accounts
fallback_to_internal = false  # Don't fall back to internal user DB

[server.auth.native]
allow_all_users = false   # Only specified users can connect
require_group = "rcp-users"  # OS group required for RCP access
permission_mapping = true # Map OS groups to RCP permissions
admin_groups = ["administrators", "wheel"] # OS groups with RCP admin privileges
```

## Permission Mapping

The native authentication provider maps OS groups to RCP permissions:

| OS Group/Role     | RCP Permission        | Description                            |
|-------------------|------------------------|----------------------------------------|
| administrators    | admin:*                | Full administrative access to RCP      |
| wheel/sudo        | admin:*                | Full administrative access to RCP      |
| staff             | connect:*              | Permission to connect to RCP server    |
| rcp-users         | connect:basic          | Basic connection permissions           |
| rcp-app-[name]    | app:[name]             | Permission to use specific application |

### Custom Permission Mappings

You can define custom mappings between OS groups and RCP permissions:

```toml
[server.auth.native.permission_mappings]
"admin" = ["admin:*", "connect:*", "app:*"]
"staff" = ["connect:*", "app:safari"]
"developers" = ["connect:*", "app:vscode", "app:terminal"]
```
|-------------------|------------------------|----------------------------------------|
| `administrators`  | `admin:*`             | Full administrative access             |
| `rcp-users`       | `connect:*`           | Basic connection privileges            |
| `rcp-app-NAME`    | `app:NAME`            | Access to specific application         |
| `rcp-api-users`   | `api:read`            | Access to read API endpoints           |
| `rcp-api-admins`  | `api:write`           | Access to modification API endpoints   |

Custom mapping rules can be defined in the configuration file.

## Implementation Details

### Platform-Specific Authentication

#### Windows

- Leverages Windows Security Support Provider Interface (SSPI)
- Supports Active Directory domain authentication
- Uses Windows Security Tokens for credential validation

#### macOS

- Integrates with Directory Services API
- Supports local accounts and Open Directory
- Optional Touch ID / Apple Watch authentication

#### Linux

- Uses Pluggable Authentication Modules (PAM)
- Supports integration with LDAP, Kerberos, SSSD
- Compatible with enterprise Linux authentication systems

### Security Considerations

1. **Privilege Management**: The RCP daemon requires appropriate system privileges to validate user accounts.
2. **Credential Security**: The implementation never stores OS credentials, validating them directly with the OS.
3. **Audit Logging**: All authentication attempts are logged with detailed information for security review.
4. **Session Management**: OS user sessions are bound to RCP sessions with appropriate timeout controls.

## User Experience

From an end-user perspective, native authentication works seamlessly:

1. User connects to RCP server using their OS username
2. Authentication happens through OS mechanisms (possibly transparent to the user)
3. RCP permissions are derived from OS group membership
4. The user experience remains consistent with other applications

## Limitations

- Different operating systems have varying authentication capabilities
- Some advanced features may be platform-specific
- Certain enterprise authentication systems may require additional configuration

## Integration Testing

To verify native authentication is working properly:

1. Configure RCP for native authentication
2. Create test users in the OS with different permission levels
3. Attempt connection with each test user
4. Verify appropriate access is granted based on OS group membership

## Future Enhancements

- Integration with biometric authentication
- Support for hardware security keys
- Advanced multi-factor authentication options
- Integration with cloud identity providers (OAuth, OIDC)
