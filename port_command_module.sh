#!/bin/bash
# Script to assist with migrating command modules from rcp-cli to rcpd

# Usage: ./port_command_module.sh <module_name>
# Example: ./port_command_module.sh app

if [ -z "$1" ]; then
  echo "Usage: $0 <module_name>"
  echo "Example: $0 app"
  exit 1
fi

MODULE_NAME="$1"
SOURCE_FILE="rcp-cli/src/commands/${MODULE_NAME}.rs"
DEST_FILE="rcpd/src/cli/commands/${MODULE_NAME}.rs"

if [ ! -f "$SOURCE_FILE" ]; then
  echo "Error: Source file $SOURCE_FILE not found"
  exit 1
fi

if [ -f "$DEST_FILE" ]; then
  echo "Warning: Destination file $DEST_FILE already exists"
  read -p "Overwrite? [y/N] " CONFIRM
  if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
    exit 1
  fi
fi

# Create destination directory if it doesn't exist
mkdir -p $(dirname "$DEST_FILE")

# Add feature gate and copy the file
echo "// filepath: $DEST_FILE
//! Command module for $MODULE_NAME
//!
//! This module contains the command handlers for $MODULE_NAME-related operations.
//! Ported from rcp-cli component as part of CLI unification.

#[cfg(feature = \"cli\")]
" > "$DEST_FILE"

# Append the original file content, but skip the first line (usually the module doc comment)
tail -n +2 "$SOURCE_FILE" >> "$DEST_FILE"

echo "Module ported to $DEST_FILE"
echo ""
echo "Next steps:"
echo "1. Update imports (rcp_cli::* should become crate::cli::*)"
echo "2. Check for compatibility with rcpd structure"
echo "3. Test the command implementation"
echo "4. Ensure proper feature gating throughout the file"
echo ""
