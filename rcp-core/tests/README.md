# Integration Tests

Integration tests for the RCP core library. These tests verify that components work together correctly.

## Structure

Tests are organized by functionality:
- `protocol_tests.rs`: Tests protocol state transitions and frame handling
- `auth_tests.rs`: Tests authentication mechanisms
- `frame_tests.rs`: Tests frame serialization/deserialization
- `command_tests.rs`: Tests command processing

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
