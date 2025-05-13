# 🦀 RCP — Rust/Remote Control Protocol

**RCP/1.0** (Rust/Remote Control Protocol) is a low-level, high-performance protocol designed to enable secure remote control of desktop applications over TCP/IP using the Rust programming language. Built for performance, portability, and flexibility, RCP is designed to be the foundation for remote app virtualization or distributed desktop protocols.

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
- 🖥️ **Tauri-based management UI** for cross-platform administration
- 📡 **RESTful management API** for integration with existing systems
- 📁 **Future support**: clipboard, file transfer, remote shell
- 🔗 **SSH-like connection strings** for simple client connections

---

## 📁 Repository Structure

```
rcp/
├── rcp-core/               # Protocol definitions, frame parsers, commands
├── rcp-client/             # RCP client, app control interface
├── rcpd/                   # Runtime daemon with integrated server and API capabilities
├── rcp-cli/                # Command-line tool for server administration only
├── rcp-admin/              # Administrative interface for server management
├── rcp-desk/               # End-user client for virtual applications
├── rcp-ws-bridge/          # (optional) WebSocket proxy for browser clients
├── examples/               # Minimal demos (spawn notepad, etc.)
└── docs/                   # Protocol spec & architecture documentation
```

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
cargo run -p rcpd

# Use the CLI to manage the daemon
cargo run -p rcp-cli -- status
```

### 🔌 Connecting to a Server

The RCP client supports several methods for connecting to a server:

#### Using SSH-like Connection Strings

Connect with a single, convenient connection string:

```bash
# Connect using SSH-style string format: [user[:pass]@]host[:port][/path]
rcp-client connect admin:secretkey@192.168.1.100:8716
```

#### Using Command-Line Parameters

Connect using host and port flags (must be specified before the connect command):

```bash
# Connect using command-line parameters (defaults to "test_key" as PSK)
rcp-client -H 127.0.0.1 -p 8716 connect

# Connect with a custom PSK specified via --psk flag
rcp-client -H 127.0.0.1 -p 8716 connect --psk customkey
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
- [RCPD (RCP Daemon)](docs/rcpd.md) - Documentation for the runtime daemon with integrated server
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
