# RCP Release Process

This document outlines the process for creating and publishing new releases of the RCP (Rust Control Protocol) project.

## Version Numbering

RCP follows [Semantic Versioning](https://semver.org/) (SemVer):

- **Major version (x)**: Incremented for incompatible API changes
- **Minor version (y)**: Incremented for backward-compatible feature additions
- **Patch version (z)**: Incremented for backward-compatible bug fixes

Example: `v1.2.3`

## Pre-Release Checklist

Before creating a new release:

1. Ensure all tests pass: `cargo test --all`
2. Update version numbers in all `Cargo.toml` files
3. Update CHANGELOG.md with all notable changes
4. Merge all planned changes into the main branch

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
4. Publish the packages to crates.io
5. Finalize the GitHub Release (changing it from draft to published)

### 3. Verify the Release

After the workflow completes:

1. Verify that all artifacts are properly uploaded to the GitHub Release
2. Check that all packages are available on crates.io
3. Verify that the GitHub Release is no longer in draft state

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

---

For any questions about the release process, please contact the project maintainers.