#!/bin/bash

# Navigate to the rcp-api directory
cd "$(dirname "$0")"

echo "===== Running RCP API Tests ====="

# Export DATABASE_URL for tests that might need it
export DATABASE_URL="sqlite::memory:"

# Run tests with full output
RUST_BACKTRACE=1 cargo test -- --nocapture

# Check test results
if [ $? -eq 0 ]; then
    echo "===== All tests passed successfully! ====="
else
    echo "===== Some tests failed! ====="
    exit 1
fi
