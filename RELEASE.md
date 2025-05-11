# RCP Release Process

This document outlines the process for creating and publishing new releases of the RCP (Rust/Remote Control Protocol) project.

## Version Numbering

RCP follows [Semantic Versioning](https://semver.org/) (SemVer):

- **Major version (x)**: Incremented for incompatible API changes
- **Minor version (y)**: Incremented for backward-compatible feature additions
- **Patch version (z)**: Incremented for backward-compatible bug fixes

Example: `v1.2.3`

## Component-Specific Configuration Files

When building release packages, configuration files are handled differently for each component:

- **Server package**: Includes the `config.toml` configuration file which contains server settings such as binding address, port, TLS configuration, and authentication parameters.
- **Client package**: Does not include `config.toml` as clients use command-line parameters or connection strings to connect to servers.
- **Bridge package**: Includes the `config.toml` configuration file as it needs connection settings to relay traffic.

This ensures that each package only contains the necessary configuration files for its operation.

## Pre-Release Checklist

Before creating a new release:

1. Ensure all tests pass: `cargo test --all`
2. Update version numbers in all `Cargo.toml` files
3. Update CHANGELOG.md with all notable changes
4. Merge all planned changes into the main branch
5. If publishing to crates.io, ensure the CRATES_IO_TOKEN secret is set in your GitHub repository settings

## Creating a Release

### 1. Create and Push a Git Tag

The automated release process is triggered by pushing a tag with the version number:

```bash
# Replace x.y.z with the actual version numbers (e.g., 0.2.1)
git tag vx.y.z
git push origin vx.y.z
```

For example:
```bash
git tag v0.2.1
git push origin v0.2.1
```

This will automatically trigger the GitHub Actions release workflow.

### 2. Release Workflow

The GitHub Actions workflow will:

1. Create a new GitHub Release as a draft
2. Build the components for each platform (Windows, Linux, macOS)
3. Upload the build artifacts to the GitHub Release
4. Publish the packages to crates.io (if CRATES_IO_TOKEN is configured)
5. Finalize the GitHub Release (changing it from draft to published)

### 3. Verify the Release

After the workflow completes:

1. Verify that all artifacts are properly uploaded to the GitHub Release
2. Check that all packages are available on crates.io
3. Verify that the GitHub Release is no longer in draft state

## Setting Up crates.io Publishing

To enable automatic publishing to crates.io during releases:

1. Log in to crates.io and get your API token from https://crates.io/me
2. In your GitHub repository, go to Settings > Secrets and variables > Actions
3. Create a new repository secret:
   - Name: `CRATES_IO_TOKEN`
   - Value: Your crates.io API token
4. This token will be used securely by the GitHub Actions workflow

## Manual Release (if needed)

If you need to manually create a release:

1. Create a new release from the GitHub web interface
2. Set the tag name to the version (e.g., `v0.2.1`)
3. Add release notes from the CHANGELOG.md
4. Upload the compiled binaries manually

## Hotfix Releases

For emergency fixes:

1. Create a hotfix branch from the tagged release
2. Make the necessary changes
3. Follow the standard release process with an incremented patch version

## Announcement

After a successful release:

1. Announce the new version in GitHub Discussions
2. Update documentation if necessary
3. Notify users through appropriate channels

## Key Features to Highlight in Releases

When announcing releases, be sure to highlight these key features:

1. **SSH-like Connection Strings**: The client supports an SSH-like connection string format (`user:pass@host:port/path`) for easier connection setup.
2. **Component-Specific Configuration**: Each component only includes configuration files it actually needs.
3. **Improved Client CLI**: Command-line interface supports both traditional flag-based parameters and connection strings.

---

For any questions about the release process, please contact the project maintainers.