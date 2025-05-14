# RCP Flutter Client

## Overview

The RCP Flutter Client is a cross-platform application for connecting to RCP servers, managing user authentication, and launching and interacting with remote applications. This document outlines the architecture, design decisions, and implementation strategy for replacing the current SDL2/egui-based client with a Flutter-based solution.

## Motivation

### Current Challenges with SDL2/egui Approach

The existing implementation using Rust with SDL2 and egui faces several challenges:

- **Rendering Limitations**: The custom integration between egui and SDL2 results in distorted and unreadable UI elements.
- **Implementation Complexity**: A complete implementation would require building a custom renderer for egui primitives (triangles, textures, text) using SDL2's APIs.
- **Limited Mobile Support**: The SDL2 approach would require significant additional work to support mobile platforms.
- **Development Overhead**: Maintaining a custom egui-SDL2 bridge requires ongoing development effort as both libraries evolve.

### Benefits of Flutter Approach

Flutter offers a comprehensive solution for building cross-platform applications with these advantages:

- **Truly Cross-Platform**: Supports iOS, Android, Windows, macOS, Linux, and web from a single codebase
- **Native Performance**: Flutter compiles to native ARM and x86 code, not interpreted like many cross-platform frameworks
- **Rich UI Components**: Extensive library of pre-built, customizable widgets
- **Hot Reload**: Fast development cycle with near-instant UI updates
- **Strong Typing**: Dart language offers static typing and null safety
- **FFI Support**: Can directly interface with native code (including Rust RCP libraries)

## Architecture

### High-Level Design

The Flutter-based RCP client follows a layered architecture:

```
┌─────────────────────────────────────┐
│             Flutter UI              │
│  (Screens, Widgets, State Management)  │
├─────────────────────────────────────┤
│         Dart Business Logic         │
│     (Application State, Services)   │
├─────────────────────────────────────┤
│           Platform Channel          │
│    (Communication Bridge to Native) │
├─────────────────────────────────────┤
│        Native Platform Code         │
│ (FFI to Rust RCP Client Libraries)  │
└─────────────────────────────────────┘
```

### Key Components

1. **UI Layer**:
   - Implements all screens (Connection, Login, App Launcher, Streaming)
   - Manages UI state and user interactions
   - Handles responsive layouts for different form factors

2. **Business Logic Layer**:
   - Manages application state
   - Implements service interfaces for RCP functionality
   - Handles data conversions between UI and native layers

3. **Native Bridge Layer**:
   - Provides FFI bindings to existing Rust RCP client libraries
   - Manages memory between Dart and Rust
   - Handles callbacks and events from native code

4. **Core RCP Layer** (existing Rust libraries):
   - Implements the RCP protocol
   - Manages connections, authentication, app launching
   - Handles streaming of remote application content

### Project Structure

```
rcp-flutter/
├── android/           # Android-specific code
├── ios/               # iOS-specific code
├── linux/             # Linux-specific code
├── macos/             # macOS-specific code
├── windows/           # Windows-specific code
├── lib/
│   ├── main.dart      # Entry point
│   ├── app.dart       # App configuration
│   ├── models/        # Data models 
│   │   ├── app_info.dart
│   │   └── user.dart
│   ├── screens/       # UI screens
│   │   ├── login_screen.dart
│   │   ├── connect_screen.dart
│   │   ├── app_launcher.dart
│   │   └── streaming_screen.dart
│   ├── services/      # Business logic
│   │   ├── rcp_service.dart  # Wrapper for RCP client
│   │   ├── auth_service.dart
│   │   └── settings_service.dart
│   ├── widgets/       # Reusable UI components
│   │   ├── app_card.dart
│   │   └── connection_form.dart
│   └── utils/         # Utilities
│       ├── constants.dart
│       └── extensions.dart
├── rust/              # Native Rust code
│   ├── Cargo.toml
│   └── src/
│       └── bridge.rs  # FFI bridge to rcp_client
└── pubspec.yaml       # Flutter package configuration
```

## Integration with Existing RCP Libraries

### FFI Bridge

To leverage the existing Rust RCP client libraries, we'll create a native bridge:

1. **Create FFI Bindings in Rust**:

```rust
// rust/src/bridge.rs
use rcp_client::{ClientBuilder, RcpClient};
use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub extern "C" fn connect_to_server(
    host: *const c_char,
    port: i32
) -> i32 {
    let host_str = unsafe { CStr::from_ptr(host) }.to_str().unwrap_or("");
    
    // Use the existing RCP client library
    let client = ClientBuilder::new()
        .host(host_str)
        .port(port as u16)
        .client_name("RCP-Flutter-Client")
        .build();
        
    // In a real implementation, store client reference and return success/error code
    0 // Success
}

#[no_mangle]
pub extern "C" fn authenticate_user(
    username: *const c_char,
    password: *const c_char
) -> i32 {
    // Implementation using RCP client authentication
    0 // Success
}

// Additional FFI functions for app listing, launching, etc.
```

2. **Access from Dart**:

```dart
// lib/services/rcp_service.dart
import 'dart:ffi';
import 'package:ffi/ffi.dart';

class RcpService {
  static final DynamicLibrary _lib = DynamicLibrary.open("librcp_bridge.so");
  
  static final _connectToServer = _lib.lookupFunction<
    Int32 Function(Pointer<Utf8>, Int32),
    int Function(Pointer<Utf8>, int)
  >('connect_to_server');
  
  Future<bool> connect(String host, int port) async {
    final hostPointer = host.toNativeUtf8();
    try {
      final result = _connectToServer(hostPointer, port);
      return result == 0;
    } finally {
      calloc.free(hostPointer);
    }
  }
  
  // Additional methods for authentication, app listing, etc.
}
```

### State Management

The Flutter client will use a similar state management approach to the current SDL2 implementation:

```dart
// lib/models/app_state.dart
class AppState extends ChangeNotifier {
  bool connected = false;
  bool authenticated = false;
  String? errorMessage;
  String username = '';
  String host = 'localhost';
  int port = 8717;
  List<AppInfo> apps = [];
  String? activeAppId;
  
  // Methods to update state and notify listeners
}
```

## UI Implementation

The Flutter client will implement the following screens, matching the functionality of the current SDL2 client:

### 1. Connection Screen

Form for server connection details, allowing the user to input:
- Server hostname/IP
- Port number 
- Connect button

### 2. Login Screen

Authentication form with:
- Username field
- Password field (obscured)
- Remember credentials checkbox
- Login button

### 3. App Launcher

Grid of available remote applications:
- Application cards with name, description
- Last used information when available
- Launch button
- User information and logout option

### 4. Application Streaming

View for the streamed application:
- Main display area for the application content
- Controls for interaction with the remote application
- Close button to terminate the session

## Implementation Strategy

### Phase 1: Project Setup (1 week)

1. Create Flutter project structure
2. Set up build configuration for all platforms
3. Configure FFI integration with Rust
4. Create basic application shell with navigation

### Phase 2: Core Functionality (2 weeks)

1. Implement FFI bindings for RCP client core functions
2. Create Dart service wrappers for RCP functionality
3. Implement connection and authentication screens
4. Add state management and error handling

### Phase 3: Application UI (2 weeks)

1. Implement app launcher with application grid
2. Create UI for application streaming
3. Add responsive design for different form factors
4. Implement platform-specific adaptations as needed

### Phase 4: Testing and Refinement (1-2 weeks)

1. Implement automated tests for core functionality
2. Perform cross-platform testing
3. Optimize performance and resource usage
4. Address platform-specific issues

## Comparison with Alternatives

| Approach | Pros | Cons |
|----------|------|------|
| Current SDL2/egui | - Pure Rust implementation<br>- Direct access to RCP libraries | - Custom rendering issues<br>- Limited mobile support<br>- Development overhead |
| Flutter | - True cross-platform<br>- Rich UI components<br>- Fast development | - Dart learning curve<br>- FFI integration complexity<br>- Larger binary size |
| Tauri + Web | - Small bundle size<br>- Web stack familiarity | - Limited mobile support<br>- Less integrated UX |
| .NET MAUI | - Microsoft-backed<br>- C# productivity | - Large runtime dependency<br>- Windows-centric |
| React Native | - Large ecosystem<br>- JavaScript familiarity | - Desktop support less mature<br>- JS ecosystem complexity |

## Getting Started

To begin implementing this approach:

1. **Install Flutter**:
   ```bash
   brew install flutter
   flutter doctor
   ```

2. **Create Project**:
   ```bash
   flutter create --platforms=ios,android,macos,windows,linux rcp_flutter
   ```

3. **Add FFI Dependencies**:
   ```yaml
   # pubspec.yaml
   dependencies:
     flutter:
       sdk: flutter
     ffi: ^2.1.0
   ```

4. **Set Up Rust Library**:
   ```bash
   cd rcp_flutter
   mkdir -p rust/src
   cd rust
   cargo init --lib
   ```

5. **Build System Integration**: Configure Flutter to build the Rust library as part of the build process.

## Conclusion

The Flutter-based approach offers a viable path forward for the RCP client, addressing the current challenges with the SDL2/egui implementation while providing a cross-platform solution with excellent UI capabilities. By leveraging the existing Rust RCP libraries through FFI, we can maintain compatibility with the core RCP functionality while delivering a modern, responsive user interface across desktop and mobile platforms.

This approach provides the best balance of platform support, performance, and development efficiency for the RCP client requirements.
