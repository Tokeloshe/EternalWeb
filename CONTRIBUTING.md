# Contributing to EternalWeb

Thank you for your interest in contributing to EternalWeb! We're building a decentralized web archiving platform for all humankind, and every contribution helps make the internet more free, egalitarian, and resilient.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). We are committed to providing a welcoming and harassment-free experience for everyone.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** the behavior
- **Expected vs. actual behavior**
- **Environment details** (OS, Rust version, etc.)
- **Screenshots** if applicable
- **Error logs** if available

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- Use a **clear and descriptive title**
- Provide a **detailed description** of the proposed feature
- Explain **why this enhancement would be useful** to most EternalWeb users
- List **alternative solutions** you've considered
- Note if you're willing to implement it yourself

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Follow the coding style** (run `cargo fmt` for Rust, `prettier` for TypeScript)
3. **Write tests** for new features
4. **Update documentation** as needed
5. **Ensure all tests pass** (`cargo test`)
6. **Write clear commit messages** following the format:
   ```
   [module] Short description

   Longer explanation if needed
   ```

## Development Process

### Setting Up Your Environment

```bash
# Clone your fork
git clone https://github.com/your-username/eternalweb.git
cd eternalweb

# Install dependencies
cargo build
cd frontend && npm install

# Run tests
cargo test
```

### Project Structure

```
eternalweb/
├── eternalweb-core/    # Core Rust library
│   └── src/
│       ├── archiving/   # Archiving engine
│       ├── networking/  # P2P networking
│       ├── security/    # Cryptography
│       ├── search/      # Search engine
│       ├── blockchain/  # DAO governance
│       ├── ai/          # ML/AI features
│       ├── collaboration/ # Collectives
│       ├── incentives/  # Reputation system
│       └── utils/       # Utilities
├── eternalweb-cli/     # CLI application
├── frontend/           # React frontend
├── extension/          # Browser extension
└── docs/               # Documentation
```

### Coding Guidelines

#### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add documentation comments (`///`) for public APIs
- Write unit tests for all functions
- Use `Result<T>` for error handling

Example:
```rust
/// Archives a web page and returns its CID
///
/// # Arguments
/// * `url` - The URL to archive
///
/// # Returns
/// The Content Identifier (CID) of the archived page
///
/// # Example
/// ```
/// let cid = snapshot_page("https://example.com").await?;
/// ```
pub async fn snapshot_page(url: &str) -> Result<Cid> {
    // Implementation
}
```

#### TypeScript/React

- Use TypeScript for type safety
- Follow React best practices and hooks
- Use functional components
- Format with Prettier
- Use meaningful component and variable names

#### Documentation

- Update README.md for user-facing changes
- Add inline code comments for complex logic
- Create docs/ entries for new features
- Include examples in documentation

### Testing

- Write unit tests in the same file as the code (Rust) or separate test files (TS)
- Aim for >80% code coverage
- Test edge cases and error conditions
- Use property-based testing where appropriate

```bash
# Run Rust tests
cargo test

# Run frontend tests
cd frontend && npm test
```

### Security Considerations

EternalWeb prioritizes security. When contributing:

- **Never commit secrets** (API keys, private keys, etc.)
- **Validate all inputs** to prevent injection attacks
- **Use parameterized queries** for database operations
- **Follow OWASP guidelines** for web security
- **Encrypt sensitive data** at rest and in transit
- **Report security issues privately** to the maintainers

### Ethical Guidelines

EternalWeb is designed for positive, ethical use:

- **No illegal content** archiving features
- **Respect copyright** and licenses
- **Promote transparency** over anonymity for malicious purposes
- **Encourage education** and knowledge sharing
- **Support accessibility** for all users, including low-bandwidth regions

### Performance

- Profile code before optimizing
- Use async/await for I/O operations
- Minimize allocations in hot paths
- Batch network requests when possible
- Consider memory usage for large datasets

## Submitting Changes

1. **Create a feature branch**: `git checkout -b feature/amazing-feature`
2. **Make your changes** following the guidelines above
3. **Test thoroughly**: Ensure all tests pass
4. **Commit with clear messages**: Explain what and why
5. **Push to your fork**: `git push origin feature/amazing-feature`
6. **Open a Pull Request**: Fill out the PR template completely

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings
```

## Community

- **GitHub Discussions**: For questions and ideas
- **Issues**: For bugs and feature requests
- **Twitter/X**: Follow @e_honiball for updates

## Recognition

Contributors are recognized in:
- The README contributors section
- Release notes for significant contributions
- On-chain badges (once blockchain features are live)

Thank you for contributing to a free and open internet! 🌍
