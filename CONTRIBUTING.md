# Contributing to RCP (Rust Control Protocol)

Thank you for your interest in contributing to RCP! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

All contributors are expected to adhere to our Code of Conduct. Please be respectful and considerate of others when participating in discussions, submitting code, or engaging with the project in any way.

## Getting Started

1. **Fork the repository**
   
   Start by forking the repository to your own GitHub account.

2. **Clone your fork**
   
   ```bash
   git clone https://github.com/YOUR_USERNAME/rcp.git
   cd rcp
   ```

3. **Set up the development environment**
   
   ```bash
   # Install Rust if you haven't already
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Build the project
   cargo build
   
   # Run tests
   cargo test
   ```

## Development Workflow

1. **Create a branch for your changes**
   
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   
   Follow the coding style and practices already established in the project.

3. **Write tests**
   
   Add tests for any new functionality or bug fixes.

4. **Run the linters and formatters**
   
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   ```

5. **Commit your changes**
   
   ```bash
   git commit -m "Description of your changes"
   ```

6. **Push your changes**
   
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a pull request**
   
   Go to the GitHub repository and create a pull request from your branch to the main branch.

## Pull Request Guidelines

- Keep PRs focused on a single issue or feature
- Include a clear description of what your changes do
- Reference any related issues
- Make sure all tests pass
- Update documentation as needed

## Coding Standards

- Follow Rust's official style guidelines
- Use meaningful variable and function names
- Comment complex code sections
- Keep functions small and focused
- Use error handling instead of panics where possible

## Project Structure

```
open-rcp/
├── rcp-core/       # Protocol definitions, frame parsers, commands
├── rcp-server/     # RCP listener, app session manager
├── rcp-client/     # TCP client, app control interface
├── rcp-ws-bridge/  # WebSocket proxy for browser clients
├── examples/       # Minimal demos
└── docs/           # Protocol spec & diagrams
```

## Documentation

When adding new features or making significant changes, please update the relevant documentation:

- **Protocol changes**: Update `docs/spec.md`
- **Architecture changes**: Update `docs/architecture.md` 
- **Implementation details**: Update `docs/implementation_guide.md`
- **Public API**: Add Rust doc comments (`///`) to public items

## Testing

We use several levels of testing:

1. **Unit tests**: Test individual components in isolation
2. **Integration tests**: Test interactions between components
3. **End-to-end tests**: Test complete workflows

When adding new functionality, please include appropriate tests.

## Release Process

1. Version numbers follow [Semantic Versioning](https://semver.org/)
2. Changes for each version are documented in `CHANGELOG.md`
3. Releases are tagged in Git and published to crates.io

## Getting Help

If you need help with anything, please:

1. Check the documentation
2. Look through existing issues
3. Join the discussion on GitHub Discussions
4. Reach out to the maintainers

## License

By contributing to RCP, you agree that your contributions will be licensed under the project's MIT/Apache-2.0 license.

---

Thank you for contributing to RCP! Your efforts help make this project better for everyone.

~ Devstroop Technologies
