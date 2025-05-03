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
- 📁 **Future support**: clipboard, file transfer, remote shell

---

## 📁 Repository Structure

```
open-rcp/
├── rcp-core/       # Protocol definitions, frame parsers, commands
├── rcp-server/     # RCP listener, app session manager
├── rcp-client/     # TCP client, app control interface
├── rcp-ws-bridge/  # (optional) WebSocket proxy for browser clients
├── examples/       # Minimal demos (spawn notepad, etc.)
└── docs/           # Protocol spec & diagrams
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

# Build server
cargo build -p rcp-server

# Run server
./target/debug/rcp-server
```

---

## 🧱 Roadmap

* [x] Define core protocol
* [x] TCP socket server/client
* [x] Launch & control remote apps
* [ ] Screen streaming (shared memory or framebuffer)
* [ ] Browser client via WebSocket bridge
* [ ] Clipboard & file share support
* [ ] WebAssembly interface for frontend

---

## 📄 License

MIT OR Apache-2.0 — your choice.

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