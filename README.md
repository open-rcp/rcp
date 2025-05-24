# 🦀 RCP — Rust/Remote Control Protocol

**RCP/1.0** (Rust/Remote Control Protocol) is a low-level, high-performance protocol designed to enable secure remote control of desktop applications over TCP/IP using the Rust programming language. Built for performance, portability, and flexibility, RCP is designed to be the foundation for remote app virtualization or distributed desktop protocols.


![CI Status](https://github.com/open-rcp/rcp/actions/workflows/ci.yml/badge.svg)
![Release Status](https://github.com/open-rcp/rcp/actions/workflows/release.yml/badge.svg)


> 🔒 Secure. ⚡ Fast. 🧩 Modular. 🦀 Rust-native.

---

## 📦 Features

- 🔌 **TCP/IP transport** with optional TLS (via `rustls`)
- 🖥️ **Remote application spawning** and control
- 📤 **Input & screen stream support**
- 🔐 **Authentication layer** (pre-shared keys or public-key auth)
- 📦 **Modular protocol structure** for easy extension
- 🛠️ **Dynamic configuration** of applications via cli/api/desk
- 🎛️ **CLI tooling** for streamlined administration
- 📡 **RESTful management API** for integration with existing systems
- 📁 **Future support**: clipboard, file transfer, remote shell
- 🔗 **SSH-like connection strings** for simple client connections

---

## 📁 Repository Structure

```
rcp/
├── rcpcore/                   # Protocol definitions, frame parsers, commands
├── rcpcli/                   # RCP client library and CLI interface
├── rcpdaemon/                   # Runtime daemon with integrated server and API 
├── examples/               # Minimal demos and example implementations
└── docs/                   # Protocol spec & architecture documentation
```

For more detailed documentation on each component:
- See [rcpcore.md](docs/rcpcore.md) for the protocol library
- See [rcpcli.md](docs/rcpcli.md) for the client library
- See [rcpdaemon.md](docs/rcpdaemon.md) for the daemon implementation
- See [architecture.md](docs/architecture.md) for overall system design
- See [development-workflow.md](docs/development-workflow.md) for development guidelines

---

## 📡 Protocol Overview — `RCP/1.0`

RCP is a binary protocol over TCP:

### 🔧 Frame Header (example)
```rust
struct RcpHeader {
    version: u8,         // always 0x01
    command: u8,         // e.g., 0x01 = LaunchApp
    payload_size: u32,   // body length
    flags: u16,          // reserved for compression, etc.
}
```

### 🎮 Command Set

| Command ID | Name        | Description                   |
| ---------- | ----------- | ----------------------------- |
| `0x01`     | LaunchApp   | Start a desktop application   |
| `0x02`     | SendInput   | Mouse/keyboard input          |
| `0x03`     | StreamFrame | Sends raw window/screen frame |
| `0xFE`     | Auth        | Auth handshake                |
| `0xFF`     | Heartbeat   | Keepalive                     |

---

## 🚀 Getting Started

### 🛠️ Prerequisites

* Rust ≥ 1.75
* Linux/Windows/macOS
* OpenSSL or Rustls (for TLS)

### 🔧 Build & Run (Example)

```bash
# Clone
git clone https://github.com/open-rcp/rcp.git
cd rcp

# Build the complete stack
cargo build

# Run the daemon
cargo run -p rcpdaemon

# Use the CLI to manage the daemon
cargo run -p rcp-cli -- status
```

### 🔌 Connecting to a Server

The RCP client supports several methods for connecting to a server:

#### Using SSH-like Connection Strings

Connect with a single, convenient connection string:

```bash
# Connect using SSH-style string format: [user[:pass]@]host[:port][/path]
rcpcli connect admin:secretkey@192.168.1.100:8716
```

#### Using Command-Line Parameters

Connect using host and port flags (must be specified before the connect command):

```bash
# Connect using command-line parameters (defaults to "test_key" as PSK)
rcpcli -H 127.0.0.1 -p 8716 connect

# Connect with a custom PSK specified via --psk flag
rcpcli -H 127.0.0.1 -p 8716 connect --psk customkey
```

### 🖥️ Using the Admin Interface

```bash
# Start the server management interface (SvelteKit+Tauri)
cargo run -p rcp-admin
```

The admin interface provides a complete interface for:
- Managing server configurations
- Monitoring active sessions
- Configuring application access
- Viewing analytics and logs
- User management

#### Web-based Admin Interface

```bash
# Start the web-based admin interface
cargo run -p rcp-admin -- --web
```

### 🖱️ Using the End-User Client

```bash
# Start the virtual application client
cargo run -p rcp-desk
```

The client app provides an intuitive interface for:
- Connecting to RCP servers
- Launching virtual applications
- Managing file transfers
- Setting preferences
- Managing sessions

---

## 🔑 Authentication Options

RCP supports several authentication methods:

- **Pre-shared key** (PSK): Simple shared secret authentication
- **Native OS Authentication**: Leverages the host operating system's user accounts
- **Public key**: SSH-like asymmetric cryptography (coming soon)
- **LDAP & OAuth**: Enterprise directory integration (coming soon)

### Native OS Authentication

The native authentication feature allows RCP to use existing OS user accounts:

```toml
# Sample native auth configuration 
[server.auth]
provider = "native"       # Use native OS authentication
required = true           # Authentication is required
psk = "customkey"         # Optional fallback for service accounts
fallback_to_internal = true  # Fall back to internal auth if needed

[server.auth.native]
allow_all_users = false   # Only specified users can connect
require_group = "staff"   # Required OS group for access
permission_mapping = true # Map OS groups to RCP permissions
admin_groups = ["admin", "wheel"]
```

This enables:
- Single sign-on with OS credentials
- Permission mapping from OS groups to RCP permissions
- Integration with enterprise directory services

See [Native Authentication Guide](docs/native-authentication-guide.md) for details.

---

## 📚 Documentation

RCP provides comprehensive documentation to help you understand, implement, and extend the protocol:

### Core Documentation
- [Project Architecture](docs/architecture.md) - Detailed architecture and components overview
- [Protocol Specification](docs/protocol-specification.md) - Technical specification of the RCP protocol
- [Development Guidelines](docs/development-guidelines.md) - Guide for implementing RCP in applications
- [Project Outline](docs/project-outline.md) - Project structure and development guidelines
- [Directory Structure](DIRECTORY_STRUCTURE.md) - Explanation of project directories and files
- [Quick Start Guide](QUICKSTART.md) - Fast path for new developers

### Component Documentation
- [rcpdaemon (RCP Daemon)](docs/rcpdaemon.md) - Documentation for the runtime daemon with integrated server
- [rcpdaemon Installation Guide](docs/rcpdaemon-installation.md) - Comprehensive guide for building and installing rcpdaemon
- [RCP CLI](docs/rcp-cli.md) - Documentation for the command-line interface (server administration)
- [RCP API](docs/rcp-api.md) - Documentation for the integrated REST API component
- [RCP Admin](docs/rcp-admin.md) - Documentation for the administrative interface
- [RCP Desk](docs/rcp-desk.md) - Documentation for the end-user client application

### Development Resources
- [BUILD](BUILD.md) - Instructions for building and developing RCP
- [Roadmap (Legacy)](docs/roadmap-legacy.md) - Project milestones and development plans
- [Contributing](CONTRIBUTING.md) - Guidelines for contributors
- [Release Notes](RELEASE.md) - Version history and release information

---

## 🤝 Contributing

Contributions, bug reports, and suggestions are welcome!
Please see [`CONTRIBUTING.md`](./CONTRIBUTING.md) to get started.

## 👥 Contributors

- **Akash Shah** - [github.com/itsalfredakku](https://github.com/itsalfredakku)

---

## 🏢 Publisher

### Devstroop Technologies

RCP is proudly developed and maintained by **Devstroop Technologies**, a forward-thinking software development company specializing in high-performance systems, infrastructure tools, and cutting-edge protocol implementations.

🌐 [devstroop.com](https://devstroop.com)

Devstroop Technologies focuses on creating robust, enterprise-grade solutions with a particular emphasis on performance-critical applications and system-level software. With expertise in Rust and systems programming, Devstroop delivers innovative solutions for modern infrastructure challenges.

---

## 🧠 Author

Made with 💻 by Akash Shah at Devstroop Technologies.
Project inspired by GraphOn / GoGlobal internals — re-imagined from scratch.

Follow updates: [github.com/open-rcp](https://github.com/open-rcp)
