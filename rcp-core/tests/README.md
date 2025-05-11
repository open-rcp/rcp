# Integration Tests

Integration tests for the RCP core library. These tests verify that components work together correctly.

## Structure

Tests are organized by functionality:
- `protocol_tests.rs`: Tests protocol state transitions and frame handling
- `auth_tests.rs`: Tests authentication mechanisms
- `frame_tests.rs`: Tests frame serialization/deserialization
- `command_tests.rs`: Tests command processing

## Test Cleanup Plan

Currently, we have duplicate test files with singular and plural names:
- auth_test.rs and auth_tests.rs
- command_test.rs and command_tests.rs
- frame_test.rs and frame_tests.rs
- protocol_test.rs and protocol_tests.rs

For better maintainability, we should consolidate these files into the plural versions:

1. Move any unique tests from singular files into their plural counterparts
2. Remove the singular test files
3. Ensure all tests are properly organized by functionality

## Running Tests

To run all tests:
```bash
cargo test
```

To run a specific test with detailed output:
```bash
cargo test test_name -- --nocapture
```

## Test Guidelines

1. Each test should focus on a single feature or requirement
2. Tests should verify both success and failure conditions
3. Edge cases should be explicitly tested
4. Large data payloads should be tested to ensure performance
5. Tests should be independent and able to run in any order

## Future Integration Tests

As the project progresses, we should also develop more integration tests:

1. **Server-Client Integration Tests**
   - Complete authentication flow
   - Command processing and responses
   - Connection handling and reconnection

2. **Performance Tests**
   - Frame processing under high load
   - Multiple simultaneous connections
   - Protocol efficiency with large payloads
