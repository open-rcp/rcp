# 🦀 RCP — Rust Control Protocol

**RCP/1.0** (Rust Control Protocol) is a low-level, high-performance protocol designed to enable secure remote control of desktop applications over TCP/IP using the Rust programming language. Built for performance, portability, and flexibility, RCP is designed to be the foundation for remote app virtualization or distributed desktop protocols.

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
├── rcp-server/             # RCP listener, app session manager
├── rcp-client/             # RCP client, app control interface
├── rcp-service/            # Runtime service with app lifecycle management
├── rcp-cli/                # Command-line interface for management/control/service
├── rcp-api/                # RESTful API for remote management
├── rcp-desk/               # Unified admin interface (SvelteKit+Tauri, Web+Desktop)
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

# Run the service
cargo run -p rcp-service

# Use the CLI to manage the service
cargo run -p rcp-cli -- status
```

### 🔌 Connecting to a Server

The RCP client supports several methods for connecting to a server:

#### Using SSH-like Connection Strings

Connect with a single, convenient connection string:

```bash
# Connect using SSH-style string format: [user[:pass]@]host[:port][/path]
./target/debug/rcp-client connect admin:secretkey@192.168.1.100:8716
```

#### Using Command-Line Parameters

Connect using host and port flags (must be specified before the connect command):

```bash
# Connect using command-line parameters (defaults to "test_key" as PSK)
./target/debug/rcp-client -H 127.0.0.1 -p 8716 connect

# Connect with a custom PSK specified via --psk flag
./target/debug/rcp-client -H 127.0.0.1 -p 8716 connect --psk customkey
```

### 🖥️ Using the Management UI

```bash
# Start the management interface (SvelteKit+Tauri)
cargo run -p rcp-desk
```

The desk management UI provides a complete interface for:
- Managing server configurations
- Monitoring active sessions
- Configuring application access
- Viewing analytics and logs
- User management

#### Web Interface

```bash
# Start the web interface
cargo run -p rcp-desk -- --web
```

#### Desktop App

```bash
# Build and install the desktop app
cargo run -p rcp-desk -- --desktop-install

# Run the desktop app
cargo run -p rcp-desk
```

---

## 🧱 Roadmap

* [x] Define core protocol
* [x] TCP socket server/client
* [x] Launch & control remote apps
* [x] SSH-like connection strings
* [ ] Runtime service architecture
* [ ] CLI management tool
* [ ] Screen streaming (shared memory or framebuffer)
* [ ] Tauri-based management UI
* [ ] RESTful management API
* [ ] Browser client via WebSocket bridge
* [ ] Clipboard & file share support
* [ ] WebAssembly interface for frontend

---

## 📄 License

RCP (Rust Control Protocol) is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

```
Copyright (c) 2024-2025 Akash Shah, Devstroop Technologies

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

The Apache License 2.0 is a permissive license that allows you to freely use, modify, distribute, and sell your software based on RCP, with some important protections:

- Patent protection: Contributors explicitly grant patent rights to users
- Trademark protection: The RCP name and logo are protected
- Attribution requirement: You must retain copyright notices and provide attribution

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