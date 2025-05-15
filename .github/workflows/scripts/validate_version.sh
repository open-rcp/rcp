#!/bin/bash
# Version validator script for RCP project
# Usage: ./validate_version.sh <version>

VERSION=$1

# Check if version is provided
if [ -z "$VERSION" ]; then
  echo "Error: No version provided"
  echo "Usage: ./validate_version.sh <version>"
  exit 1
fi

# Validate version format (vX.Y.Z or vX.Y.Z-suffix)
if ! [[ $VERSION =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
  echo "Error: Invalid version format: $VERSION"
  echo "Version must be in format v1.2.3 or v1.2.3-alpha"
  exit 1
fi

echo "✓ Version $VERSION is valid"
exit 0
