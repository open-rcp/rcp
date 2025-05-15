#!/bin/bash
# Changelog generator script for RCP project
# Usage: ./generate_changelog.sh <previous_tag> <current_version>

# Set defaults
PREV_TAG=${1:-$(git describe --tags --abbrev=0 2>/dev/null || echo "HEAD~100")}
VERSION=${2:-$(git describe --tags --abbrev=0 || echo "v0.0.0")}
OUTPUT_FILE="CHANGELOG.md.new"

echo "Generating changelog from $PREV_TAG to HEAD for version $VERSION"

# Create header
echo "## $VERSION ($(date '+%Y-%m-%d'))" > $OUTPUT_FILE
echo "" >> $OUTPUT_FILE

# Features
echo "### Features" >> $OUTPUT_FILE
git log $PREV_TAG..HEAD --pretty=format:"* %s" --grep="^feat" | grep -v "^$" | sort >> $OUTPUT_FILE || echo "* No new features in this release" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE

# Bug fixes
echo "### Bug Fixes" >> $OUTPUT_FILE
git log $PREV_TAG..HEAD --pretty=format:"* %s" --grep="^fix" | grep -v "^$" | sort >> $OUTPUT_FILE || echo "* No bug fixes in this release" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE

# Improvements
echo "### Improvements" >> $OUTPUT_FILE
git log $PREV_TAG..HEAD --pretty=format:"* %s" --grep="^improve\|^refactor\|^perf" | grep -v "^$" | sort >> $OUTPUT_FILE || echo "* No improvements in this release" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE

# Documentation changes
echo "### Documentation" >> $OUTPUT_FILE
git log $PREV_TAG..HEAD --pretty=format:"* %s" --grep="^docs" | grep -v "^$" | sort >> $OUTPUT_FILE || echo "* No documentation changes in this release" >> $OUTPUT_FILE

echo "Generated changelog saved to $OUTPUT_FILE"
