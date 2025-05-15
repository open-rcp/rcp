# Workflow Scripts

This directory contains helper scripts used by GitHub Actions workflows.

## Available Scripts

### `generate_changelog.sh`

Generates a formatted changelog based on Git commit history.

**Usage:**
```bash
./generate_changelog.sh [previous_tag] [current_version]
```

**Example:**
```bash
./generate_changelog.sh v0.1.0 v0.2.0
```

**Output:**
- Creates a file `CHANGELOG.md.new` with formatted changelog entries

### `validate_version.sh`

Validates that a version string follows the project's versioning conventions.

**Usage:**
```bash
./validate_version.sh <version>
```

**Example:**
```bash
./validate_version.sh v1.2.3
```

**Output:**
- Returns exit code 0 if valid, non-zero if invalid
- Displays validation result message

### `validate_workflows.sh`

Validates GitHub Actions workflow files for common issues.

**Usage:**
```bash
./validate_workflows.sh
```

**Checks:**
- YAML syntax errors
- Hardcoded secrets
- Outdated GitHub Actions versions

## Integration with GitHub Actions

These scripts are used in the CI/CD workflows:

1. `generate_changelog.sh` - Used in the `changelog.yml` workflow for generating release notes
2. `validate_version.sh` - Used in the `release.yml` workflow to ensure version format correctness
3. `validate_workflows.sh` - Can be run locally before committing workflow changes

## Best Practices

- Always make scripts executable: `chmod +x script_name.sh`
- Use proper error handling and exit codes
- Include helpful output messages
- Maintain cross-platform compatibility when possible
