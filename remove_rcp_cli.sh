#!/bin/bash
# Script to safely remove rcp-cli component after migration is complete

# Set the base directory
BASE_DIR="$(pwd)"
BACKUP_DIR="${BASE_DIR}/backup/rcp-cli-backup-$(date +%Y%m%d)"

# Check if we're in the right directory
if [ ! -d "${BASE_DIR}/rcp-cli" ] || [ ! -d "${BASE_DIR}/rcpd" ]; then
  echo "Error: This script must be run from the project root directory"
  echo "Expected structure: <project_root>/rcp-cli and <project_root>/rcpd"
  exit 1
fi

# Verification check - ensure that rcpd has CLI feature
if ! grep -q "cli = \[" "${BASE_DIR}/rcpd/Cargo.toml"; then
  echo "Error: rcpd does not appear to have the 'cli' feature defined"
  echo "Migration may not be complete. Aborting."
  exit 1
fi

# Verify that all command modules have been migrated
echo "Checking command module migration status..."
MISSING_MODULES=0
for MODULE in app user session completions diag; do
  if [ ! -f "${BASE_DIR}/rcpd/src/cli/commands/${MODULE}.rs" ]; then
    echo "Warning: ${MODULE}.rs module appears to be missing in rcpd"
    MISSING_MODULES=$((MISSING_MODULES + 1))
  fi
done

if [ $MISSING_MODULES -gt 0 ]; then
  echo "Warning: $MISSING_MODULES command modules appear to be missing."
  read -p "Continue anyway? [y/N] " CONTINUE
  if [ "$CONTINUE" != "y" ] && [ "$CONTINUE" != "Y" ]; then
    exit 1
  fi
fi

# Create backup
echo "Creating backup of rcp-cli at ${BACKUP_DIR}..."
mkdir -p "${BACKUP_DIR}"
cp -r "${BASE_DIR}/rcp-cli" "${BACKUP_DIR}/"

# Update Cargo.toml to remove rcp-cli from workspace
echo "Updating Cargo.toml to remove rcp-cli from workspace members..."
sed -i.bak '/rcp-cli/d' "${BASE_DIR}/Cargo.toml"

# Remove rcp-cli directory
echo "Removing rcp-cli directory..."
rm -rf "${BASE_DIR}/rcp-cli"

# Update documentation to reflect removal
echo "Updating README.md to reflect removal of rcp-cli..."
sed -i.bak 's/cargo run -p rcp-cli -- status/cargo run -p rcpd --features cli -- service status/g' "${BASE_DIR}/README.md"

# Report completion
echo "Removal of rcp-cli component is complete."
echo "Backup created at: ${BACKUP_DIR}"
echo "Next steps:"
echo "1. Update any remaining documentation referring to rcp-cli"
echo "2. Test the rcpd CLI functionality"
echo "3. Update any build scripts or workflows if necessary"
echo ""
echo "If you need to restore the backup, run:"
echo "cp -r ${BACKUP_DIR}/rcp-cli ${BASE_DIR}/"
echo ""
