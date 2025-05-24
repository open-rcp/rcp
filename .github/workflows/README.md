# GitHub Actions Workflows Documentation

This document provides an overview of the GitHub Actions workflows used in the RCP project.

## Overview of Workflows

1. **CI Workflow** (`ci.yml`): Handles continuous integration tasks including building, testing, and code quality checks.
2. **Release Workflow** (`release.yml`): Creates releases and builds platform-specific packages.
3. **Changelog Workflow** (`changelog.yml`): Automatically generates and updates the changelog based on commit history.

## Workflow Optimizations

The GitHub Actions workflows have been optimized in the following ways:

### CI Workflow (`ci.yml`)

- **Matrix Builds**: Uses a build matrix for efficient multi-platform testing (Linux, Windows, macOS)
- **Smart Caching**:
  - Platform-specific caching with well-defined cache keys based on `Cargo.lock`
  - Separate project-specific caching for documentation and dependencies
  - Conditional cache saving based on branch to avoid unnecessary cache updates
- **Ordered Execution**:
  - Fast checks (formatting, clippy) run first to provide quick feedback
  - Actual builds run only after checks pass
  - Tests can be conditionally skipped with `[skip-tests]` in the commit message
- **Build Artifacts**:
  - Debug builds are archived and can be reused in the release workflow
  - Artifacts only retained for 2 days to save storage space

### Release Workflow (`release.yml`)

- **Version Management**:
  - Automatic version detection from Git tags
  - Version format validation ensures consistent versioning
  - Reuses cached builds from CI when possible
- **Release Notes**:
  - Automatic generation based on conventional commit messages
  - Properly categorized into features, fixes, and improvements
  - Uses Git history since previous tag for accurate changelogs
- **Packaging**:
  - Platform-specific packaging for each component (rcpcore, rcpcli, rcpdaemon)
  - Consistent artifact naming and formatting
  - Release published as draft first for review, then published when all builds succeed

### Changelog Workflow (`changelog.yml`)

- **Automatic Updates**:
  - Triggered on pushes to master and PR merges
  - Generates formatted changelog entries based on conventional commits
  - Only runs when actual significant changes are detected
- **Version Detection**:
  - Automatic version detection from Git tags or Cargo.toml
  - Supports manual version specification through workflow dispatch
- **PR or Direct Commit**:
  - Creates PR for changelog updates when triggered manually
  - Commits directly to master when triggered automatically
  - Fallback mechanism for PR creation if direct method fails

## Scripts

Helper scripts are located in the `.github/workflows/scripts` directory:

- `generate_changelog.sh`: Generates formatted changelog entries from Git history
- `validate_version.sh`: Validates version string format and compatibility

## Conventional Commit Types

Commits should follow the conventional commit format for proper changelog generation:

- `feat`: New features
- `fix`: Bug fixes
- `improve`/`refactor`/`perf`: Code improvements
- `docs`: Documentation changes

## Usage

These workflows run automatically based on their trigger conditions, but can also be manually triggered:

- Changelog Update: Use workflow dispatch with a specific version to update the changelog
- Release Creation: Push a tag starting with 'v' or use workflow dispatch with a specific version
