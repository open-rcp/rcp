# RCP/1.0 Protocol Specification

## Overview

The Rust/Remote Control Protocol (RCP) is designed for secure remote application control over TCP/IP networks. This document describes version 1.0 of the protocol.

## Protocol Design Goals

- **Performance**: Minimal overhead, efficient binary encoding
- **Security**: Authentication and encryption built-in
- **Extensibility**: Modular frame structure for future extensions
- **Compatibility**: Cross-platform support (Windows, Linux, macOS)

## Transport Layer

RCP operates over TCP/IP with optional TLS for encryption. The default port is 9277.

## Frame Structure

### Header Format

Every RCP frame begins with a header:

```
+--------+--------+----------------+----------------+
| Byte 0 | Byte 1 | Bytes 2-5      | Bytes 6-7      |
+========+========+================+================+
|Version |Command |Payload Size    |Flags           |
|(0x01)  |ID      |(u32, LE)       |(u16, LE)       |
+--------+--------+----------------+----------------+
```

- **Version**: Always 0x01 for RCP/1.0
- **Command ID**: Identifies the frame type
- **Payload Size**: Length of data following the header (little-endian)
- **Flags**: Reserved for future use (compression, etc.)

### Command Types

| Command ID | Name              | Description                            |
|------------|-------------------|----------------------------------------|
| `0x01`     | LaunchApp         | Start a desktop application            |
| `0x02`     | SendInput         | Mouse/keyboard input                   |
| `0x03`     | StreamFrame       | Sends raw window/screen frame          |
| `0x04`     | ResizeWindow      | Change remote window dimensions        |
| `0x05`     | ClipboardData     | Clipboard transfer                     |
| `0x06`     | FileTransfer      | File transfer command                  |
| `0x07`     | AudioData         | Audio stream frame                     |
| `0x08`     | DisplayInfo       | Send display configuration             |
| `0x09`     | CursorPosition    | Update cursor position                 |
| `0x0A`     | PermissionRequest | Request specific permission            |
| `0x0B`     | ServiceSubscribe  | Subscribe to a specific service        |
| `0x0C`     | VideoQuality      | Adjust video stream quality parameters |
| `0x0D`     | PrivacyMode       | Toggle privacy mode on/off             |
| `0x0E`     | WindowFocus       | Change focus to specific window        |
| `0xF0`     | Ping              | Connection check                       |
| `0xF1`     | Error             | Error notification                     |
| `0xFE`     | Auth              | Authentication handshake               |
| `0xFF`     | Heartbeat         | Connection keepalive                   |

## Authentication

RCP supports multiple authentication methods:

1. **Pre-shared keys**: Simple symmetric encryption
2. **Public-key authentication**: RSA or Ed25519 keys
3. **Two-factor authentication**: Time-based OTP for additional security

The authentication process follows these steps:

```
┌─────────┐                              ┌──────────┐
│ Client  │                              │  Server  │
└────┬────┘                              └─────┬────┘
     │                                         │
     │       1. Connection Request             │
     │ ──────────────────────────────────────► │
     │                                         │
     │       2. Authentication Challenge       │
     │ ◄────────────────────────────────────── │
     │                                         │
     │       3. Authentication Response        │
     │ ──────────────────────────────────────► │
     │                                         │
     │       4. Auth Result + Session Info     │
     │ ◄────────────────────────────────────── │
     │                                         │
```

## Session Management

RCP implements a robust session management system:

1. **Session Creation**: Each authenticated connection receives a unique session ID
2. **Session Tracking**: Server maintains state for active sessions 
3. **Session Timeout**: Idle sessions are terminated after configurable period
4. **Session Resumption**: Quick reconnection to existing sessions
5. **Session Audit**: All session activities are logged for security purposes

## Detailed Command Specifications

### LaunchApp (0x01)

Starts a new application on the server.

```
Payload:
+----------------+----------------+----------------+------------------+
| Bytes 0-3      | Bytes 4-7      | Bytes 8-11     | Bytes 12+        |
+================+================+================+==================+
|Flags (u32, LE) |Path Length     |Args Length     |Application Path  |
|                |(u32, LE)       |(u32, LE)       |+ Args (UTF-8)    |
+----------------+----------------+----------------+------------------+
```

Flags:
- 0x00000001: Run as administrator/elevated
- 0x00000002: Start minimized
- 0x00000004: Start maximized
- 0x00000008: Wait for exit

### SendInput (0x02)

Sends keyboard or mouse input events.

```
Payload:
+----------------+----------------+----------------+
| Byte 0         | Byte 1         | Bytes 2+       |
+================+================+================+
|Input Type      |Input Modifiers |Input Data      |
|(0=kb, 1=mouse) |                |                |
+----------------+----------------+----------------+
```

Mouse Input Data:
```
+----------------+----------------+----------------+----------------+
| Bytes 0-3      | Bytes 4-7      | Byte 8         | Bytes 9-10     |
+================+================+================+================+
|X position      |Y position      |Button mask     |Wheel delta     |
|(i32, LE)       |(i32, LE)       |                |(i16, LE)       |
+----------------+----------------+----------------+----------------+
```

Keyboard Input Data:
```
+----------------+----------------+----------------+
| Bytes 0-3      | Byte 4         | Byte 5         |
+================+================+================+
|Key code        |Key state       |Reserved        |
|(u32, LE)       |(0=up, 1=down)  |                |
+----------------+----------------+----------------+
```

### StreamFrame (0x03)

Transfers screen capture data.

```
Payload:
+-------+-------+-------+-------+-------+-------+----------------+
|Bytes  |Bytes  |Bytes  |Bytes  |Bytes  |Bytes  | Bytes 18+      |
|0-3    |4-7    |8-11   |12-13  |14-15  |16-17  |                |
+=======+=======+=======+=======+=======+=======+================+
|Width  |Height |Display|Format |Flags  |Quality|Image Data      |
|(u32)  |(u32)  |ID(u32)|(u16)  |(u16)  |(u16)  |                |
+-------+-------+-------+-------+-------+-------+----------------+
```

Format values:
- 0: Raw RGB
- 1: JPEG
- 2: PNG
- 3: VP8
- 4: VP9
- 5: H264
- 6: H265/HEVC

Flags:
- 0x0001: Keyframe
- 0x0002: Contains cursor
- 0x0004: Partial update

### ClipboardData (0x05)

Transfers clipboard content.

```
Payload:
+----------------+----------------+----------------+
| Byte 0         | Bytes 1-4      | Bytes 5+       |
+================+================+================+
|Format          |Data Length     |Clipboard Data  |
|(0=text, 1=file)|(u32, LE)       |                |
+----------------+----------------+----------------+
```

### FileTransfer (0x06)

Handles file transfer operations.

```
Payload:
+----------------+----------------+----------------+----------------+
| Byte 0         | Bytes 1-8      | Bytes 9-12     | Bytes 13+      |
+================+================+================+================+
|Operation       |File Size       |Path Length     |File Path       |
|                |(u64, LE)       |(u32, LE)       |(UTF-8)         |
+----------------+----------------+----------------+----------------+
```

Operations:
- 0x01: Begin file send
- 0x02: File chunk
- 0x03: End file send
- 0x04: File request
- 0x05: Directory listing request
- 0x06: Directory listing response

### PrivacyMode (0x0D)

Toggle privacy mode on or off.

```
Payload:
+----------------+----------------+----------------+
| Byte 0         | Bytes 1-4      | Bytes 5+       |
+================+================+================+
|State           |Message Length  |Message         |
|(0=off, 1=on)   |(u32, LE)       |(UTF-8)         |
+----------------+----------------+----------------+
```

## Error Handling

RCP implements a structured error handling mechanism:

```
+----------------+----------------+----------------+
| Byte 0         | Byte 1         | Bytes 2+       |
+================+================+================+
|Error Code      |Error Severity  |Error Message   |
|                |(0-3)           |(UTF-8)         |
+----------------+----------------+----------------+
```

Error severity levels:
- 0: Information
- 1: Warning
- 2: Error
- 3: Fatal

Common error codes:
- 0x01: Authentication failure
- 0x02: Permission denied
- 0x03: Invalid command
- 0x04: Protocol version mismatch
- 0x05: Internal server error
- 0x06: Resource not found
- 0x07: Operation timeout
- 0x08: Operation not supported

## Quality of Service

RCP incorporates quality of service features:

1. **Bandwidth monitoring**: Tracks available bandwidth
2. **Adaptive encoding**: Changes video quality based on network conditions
3. **Frame prioritization**: Ensures UI responsiveness
4. **Connection health checks**: Regular ping/heartbeat messages

```
+----------------+----------------+----------------+
| Bytes 0-3      | Bytes 4-7      | Bytes 8-11     |
+================+================+================+
|Current BPS     |Latency (ms)    |Packet Loss (%) |
|(u32, LE)       |(u32, LE)       |(u32, LE)       |
+----------------+----------------+----------------+
```

## Versioning and Compatibility

RCP version 1.0 is the initial stable version. Future versions will maintain backward compatibility when possible.

## Security Considerations

- All production deployments should use TLS encryption
- Auth credentials should be rotated regularly
- Implementation should rate-limit failed auth attempts
- Application launching should respect server-side permissions
- Session keys should never be reused across connections
- Input validation should be performed on all received data
- Clipboard and file transfer should require explicit permissions
