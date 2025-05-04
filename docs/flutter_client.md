# RCP Flutter Client Integration

This document outlines the integration of RCP functionality into Flutter applications through Foreign Function Interface (FFI).

## Overview

The RCP Flutter integration enables developers to build cross-platform mobile and desktop applications that can connect to RCP servers, leveraging the full power of the RCP protocol in a Flutter-friendly way.

## Architecture

The integration uses a layered architecture:

1. **Native RCP Client**: The core Rust implementation of the RCP client
2. **FFI Layer**: C-compatible wrapper functions exposed by the Rust library
3. **Dart FFI Bindings**: Generated Dart code that calls into the native library
4. **Flutter Service Layer**: Dart classes that provide Flutter-friendly APIs
5. **UI Components**: Pre-built Flutter widgets for common RCP functionality

```
┌─────────────────┐
│  Flutter UI     │
├─────────────────┤
│  Flutter        │
│  Service Layer  │
├─────────────────┤
│  Dart FFI       │
│  Bindings       │
├─────────────────┤
│  FFI Layer      │
│  (C API)        │
├─────────────────┤
│  Native RCP     │
│  Client (Rust)  │
└─────────────────┘
```

## FFI Layer

The FFI layer exposes C-compatible functions that can be called from Dart. These functions handle:

1. Client connection and authentication
2. Service subscription
3. Data transmission and reception
4. Event handling
5. Resource cleanup

### Key FFI Functions

```rust
// Client lifecycle management
fn rcp_client_create(config_json: *const c_char) -> *mut RcpClientHandle;
fn rcp_client_destroy(client: *mut RcpClientHandle);
fn rcp_client_connect(client: *mut RcpClientHandle) -> i32;
fn rcp_client_disconnect(client: *mut RcpClientHandle) -> i32;
fn rcp_client_authenticate(client: *mut RcpClientHandle) -> i32;
fn rcp_client_get_status(client: *mut RcpClientHandle) -> i32;

// Service management
fn rcp_service_subscribe(client: *mut RcpClientHandle, service_type: i32) -> *mut RcpServiceHandle;
fn rcp_service_unsubscribe(service: *mut RcpServiceHandle) -> i32;

// Display service
fn rcp_display_get_frame(service: *mut RcpServiceHandle, buffer: *mut u8, buffer_size: usize) -> i32;
fn rcp_display_set_quality(service: *mut RcpServiceHandle, quality: i32) -> i32;

// Input service
fn rcp_input_send_key(service: *mut RcpServiceHandle, key_code: i32, down: bool) -> i32;
fn rcp_input_send_mouse_move(service: *mut RcpServiceHandle, x: i32, y: i32) -> i32;
fn rcp_input_send_mouse_button(service: *mut RcpServiceHandle, button: i32, down: bool) -> i32;

// App service
fn rcp_app_launch_application(service: *mut RcpServiceHandle, app_path: *const c_char, args: *const c_char) -> i32;

// Clipboard service
fn rcp_clipboard_send_data(service: *mut RcpServiceHandle, data: *const c_char) -> i32;
fn rcp_clipboard_get_data(service: *mut RcpServiceHandle, buffer: *mut c_char, buffer_size: usize) -> i32;

// Event handling
fn rcp_register_event_callback(client: *mut RcpClientHandle, callback: extern fn(event_type: i32, data: *const c_char)) -> i32;
fn rcp_process_events(client: *mut RcpClientHandle) -> i32;
```

## Dart FFI Bindings

The Dart FFI bindings are generated using `ffigen` and provide a type-safe interface to the native functions.

Example binding:

```dart
import 'dart:ffi' as ffi;
import 'package:ffi/ffi.dart';

class RcpClientBindings {
  final ffi.DynamicLibrary _lib;
  
  RcpClientBindings(String libPath) : _lib = ffi.DynamicLibrary.open(libPath);
  
  late final RcpClientCreate rcpClientCreate = _lib
      .lookup<ffi.NativeFunction<ffi.Pointer<ffi.Void> Function(ffi.Pointer<Utf8>)>>('rcp_client_create')
      .asFunction();
      
  late final RcpClientDestroy rcpClientDestroy = _lib
      .lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>('rcp_client_destroy')
      .asFunction();
  
  // Additional function bindings...
}
```

## Flutter Service Layer

The Flutter service layer wraps the FFI bindings to provide a more Flutter-friendly API:

```dart
class RcpClient {
  final RcpClientBindings _bindings;
  late final ffi.Pointer<ffi.Void> _clientHandle;
  
  RcpClient(this._bindings) {
    final configJson = json.encode({
      'host': 'localhost',
      'port': 8716,
      'clientName': 'Flutter Client',
      'authMethod': 'psk',
      'psk': 'customkey'
    });
    
    final configPtr = configJson.toNativeUtf8();
    _clientHandle = _bindings.rcpClientCreate(configPtr);
    malloc.free(configPtr);
  }
  
  Future<bool> connect() async {
    return _bindings.rcpClientConnect(_clientHandle) == 0;
  }
  
  Future<bool> authenticate() async {
    return _bindings.rcpClientAuthenticate(_clientHandle) == 0;
  }
  
  // Additional methods...
  
  void dispose() {
    _bindings.rcpClientDestroy(_clientHandle);
  }
}

class DisplayService {
  final RcpClientBindings _bindings;
  final ffi.Pointer<ffi.Void> _serviceHandle;
  
  DisplayService._(this._bindings, this._serviceHandle);
  
  static Future<DisplayService> subscribe(RcpClient client) async {
    final handle = client._bindings.rcpServiceSubscribe(
      client._clientHandle, 
      ServiceType.display.index
    );
    return DisplayService._(client._bindings, handle);
  }
  
  Future<Uint8List?> getFrame() async {
    // Implementation...
  }
  
  Future<bool> setQuality(int quality) async {
    return _bindings.rcpDisplaySetQuality(_serviceHandle, quality) == 0;
  }
  
  void dispose() {
    _bindings.rcpServiceUnsubscribe(_serviceHandle);
  }
}

// Other service classes...
```

## UI Components

The Flutter package includes pre-built widgets for common RCP functionality:

```dart
// Remote display widget
class RemoteDisplay extends StatefulWidget {
  final DisplayService service;
  
  RemoteDisplay({required this.service});
  
  @override
  _RemoteDisplayState createState() => _RemoteDisplayState();
}

// Input handler widget
class InputHandler extends StatelessWidget {
  final InputService service;
  final Widget child;
  
  InputHandler({required this.service, required this.child});
  
  // Implementation...
}

// Application launcher widget
class AppLauncher extends StatelessWidget {
  final AppService service;
  
  AppLauncher({required this.service});
  
  // Implementation...
}

// Additional widgets...
```

## Usage Example

Here's how to use the RCP Flutter integration in an application:

```dart
import 'package:flutter/material.dart';
import 'package:rcp_flutter/rcp_flutter.dart';

void main() {
  runApp(RcpFlutterDemo());
}

class RcpFlutterDemo extends StatefulWidget {
  @override
  _RcpFlutterDemoState createState() => _RcpFlutterDemoState();
}

class _RcpFlutterDemoState extends State<RcpFlutterDemo> {
  late RcpClient _client;
  DisplayService? _displayService;
  InputService? _inputService;
  AppService? _appService;
  
  bool _isConnected = false;
  bool _isAuthenticated = false;
  
  @override
  void initState() {
    super.initState();
    _initializeRcp();
  }
  
  Future<void> _initializeRcp() async {
    // Initialize the client
    _client = RcpClientManager.instance.createClient(
      host: '192.168.1.50',
      port: 8716,
      clientName: 'Flutter Demo App',
      authMethod: RcpAuthMethod.psk,
      psk: 'customkey'
    );
    
    // Connect to the server
    final connected = await _client.connect();
    if (connected) {
      setState(() {
        _isConnected = true;
      });
      
      // Authenticate
      final authenticated = await _client.authenticate();
      if (authenticated) {
        setState(() {
          _isAuthenticated = true;
        });
        
        // Subscribe to services
        _displayService = await DisplayService.subscribe(_client);
        _inputService = await InputService.subscribe(_client);
        _appService = await AppService.subscribe(_client);
      }
    }
  }
  
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: Text('RCP Flutter Demo'),
          actions: [
            if (_isAuthenticated)
              IconButton(
                icon: Icon(Icons.power_settings_new),
                onPressed: () async {
                  await _client.disconnect();
                  setState(() {
                    _isConnected = false;
                    _isAuthenticated = false;
                  });
                },
              ),
          ],
        ),
        body: _isAuthenticated
            ? InputHandler(
                service: _inputService!,
                child: RemoteDisplay(service: _displayService!),
              )
            : Center(
                child: _isConnected
                    ? CircularProgressIndicator()
                    : ElevatedButton(
                        child: Text('Connect to RCP Server'),
                        onPressed: _initializeRcp,
                      ),
              ),
        floatingActionButton: _isAuthenticated
            ? FloatingActionButton(
                child: Icon(Icons.apps),
                onPressed: () {
                  showDialog(
                    context: context,
                    builder: (context) => AppLauncherDialog(service: _appService!),
                  );
                },
              )
            : null,
      ),
    );
  }
  
  @override
  void dispose() {
    _displayService?.dispose();
    _inputService?.dispose();
    _appService?.dispose();
    _client.dispose();
    super.dispose();
  }
}
```

## Building and Packaging

To build the RCP Flutter integration:

1. Compile the native library for each target platform:
   - Android: armeabi-v7a, arm64-v8a, x86, x86_64
   - iOS: arm64
   - macOS: x86_64, arm64
   - Windows: x86_64
   - Linux: x86_64

2. Package the native libraries with the Flutter plugin:

```
rcp_flutter/
├── android/
│   └── src/
│       └── main/
│           ├── kotlin/...
│           └── jniLibs/
│               ├── armeabi-v7a/librcp_client.so
│               ├── arm64-v8a/librcp_client.so
│               ├── x86/librcp_client.so
│               └── x86_64/librcp_client.so
├── ios/
│   ├── Classes/...
│   └── Frameworks/
│       └── librcp_client.dylib
├── macos/
│   ├── Classes/...
│   └── Frameworks/
│       └── librcp_client.dylib
├── windows/
│   ├── include/...
│   └── rcp_client.dll
├── linux/
│   ├── include/...
│   └── librcp_client.so
└── lib/
    ├── src/...
    └── rcp_flutter.dart
```

## Performance Considerations

1. **Display Updates**: Use texture-based rendering for optimal performance
2. **Background Processing**: Handle RCP events in isolates where possible
3. **Memory Management**: Properly free native resources
4. **Battery Usage**: Implement power-saving modes for mobile devices
5. **Network Efficiency**: Adapt quality based on network conditions

## Platform-specific Considerations

### Android

- Request necessary permissions in the manifest:
  - `INTERNET` for network access
  - `FOREGROUND_SERVICE` for background operation

### iOS

- Configure `Info.plist` for network usage
- Handle app lifecycle events appropriately

### Windows/macOS/Linux

- Support for multiple monitors
- Keyboard shortcuts integration

## Security Considerations

1. **Secure Storage**: Store authentication credentials securely using platform-specific secure storage
2. **TLS**: Enforce TLS for all connections
3. **Certificate Validation**: Implement proper certificate validation
4. **User Privacy**: Request minimal permissions

## Testing

1. **Unit Tests**: Test FFI bindings and service layer
2. **Integration Tests**: Test connection to real RCP servers
3. **UI Tests**: Test the user interface components
4. **Performance Tests**: Validate performance across devices

## Error Handling

Implement proper error handling for:

1. Connection failures
2. Authentication errors
3. Service subscription failures
4. Native crashes

Example:

```dart
try {
  await _client.connect();
} on RcpConnectionException catch (e) {
  showDialog(
    context: context,
    builder: (context) => AlertDialog(
      title: Text('Connection Error'),
      content: Text('Failed to connect: ${e.message}'),
      actions: [
        TextButton(
          child: Text('Retry'),
          onPressed: () => _initializeRcp(),
        ),
        TextButton(
          child: Text('Cancel'),
          onPressed: () => Navigator.pop(context),
        ),
      ],
    ),
  );
}
```

## Implementation Roadmap

1. **Phase 1: Core FFI Layer**
   - Basic client operations
   - Display and input services
   - Simple demo application

2. **Phase 2: Enhanced Services**
   - App service
   - Clipboard service
   - File transfer service
   - Better error handling

3. **Phase 3: Advanced Features**
   - Custom UI components
   - Optimized rendering
   - Configuration utilities
   - Documentation and examples

4. **Phase 4: Platform Support**
   - Mobile-specific optimizations
   - Desktop-specific features
   - Platform-specific examples

## Next Steps

1. Create the FFI wrapper for the RCP client
2. Generate Dart bindings
3. Implement the Flutter service layer
4. Build cross-platform demo applications