# AGENTS.md

## Project Overview

**DAM (Desperately Avoiding Maliciousness)** is a Rust CLI tool that wraps [OpenCode](https://opencode.ai) in Docker containers to safely run AI-generated code. It provides commands to build Docker images, run interactive containers, manage ACP servers, and teardown resources.

## Build, Lint, and Test Commands

### Building
```bash
cargo build --release    # Build optimized release binary
cargo build              # Build debug binary
cargo install --path .   # Install to ~/.cargo/bin
```

### Running
```bash
cargo run -- build --stack java    # Run build command
cargo run -- run                   # DO NOT RUN
cargo run -- --help                # Show CLI help
```

### Testing
```bash
cargo test                        # Run all tests
```

### Linting & Formatting
```bash
cargo fmt                         # Format code
cargo fmt -- --check              # Check formatting without changes
cargo clippy                      # Run linter
cargo clippy -- -D warnings       # Treat warnings as errors
```

### Other
```bash
cargo check                       # Type-check without compiling
cargo doc --no-deps               # Generate documentation
cargo audit                       # Check for vulnerabilities (if installed)
```

## Code Style Guidelines

### General Principles

- **Clarity over cleverness**: Write code that's easy to understand and maintain
- **Small, focused modules**: Each module should have a single responsibility
- **Prefer explicit over implicit**: Clear naming and structure over clever tricks
- **No unnecessary comments**: Code should be self-documenting (exception: public API docs)

### Module Organization

The project uses **Hexagonal Architecture** (Ports and Adapters):

```
src/
├── adapters/          # Driven adapters (implementations)
│   └── driven/        # Concrete implementations (Docker CLI, tests)
├── application/       # Application layer
│   ├── services/      # Application services
│   └── use_cases/     # Use case handlers
├── domain/            # Domain layer
│   ├── entities/      # Domain entities
│   ├── presets/       # Stack/image presets
│   ├── services/      # Domain services
│   └── value_objects/ # Value objects
├── ports/             # Port interfaces (traits)
│   └── outbound/      # Outbound port traits
├── main.rs            # Entry point
└── lib.rs             # Library root with public exports
```

### Error Handling

- Use `anyhow::Result<T>` for application-level error handling
- Use `thiserror` for library-level custom error types when needed
- Return `Result<()>` for functions that can fail but don't need to return a value
- Use `anyhow::bail!("message")` for early returns on errors
- Use `?` operator for propagating errors

```rust
pub fn build(&self, stack: &str) -> Result<()> {
    let image = Image::default_dam_image(dockerfile_path, build_context)?;
    let image_to_build = ImageBuilder::with_stack(&image, stack)?;
    docker.build_image(...)?;
    Ok(())
}
```

### Async and Concurrency

- Use `std::sync::Arc<dyn DockerClient>` for shared Docker client across handlers
- Traits should include `Send + Sync` bounds when appropriate
- Prefer synchronous code unless async is specifically needed

### Testing

- Tests are co-located in `#[cfg(test)]` modules at the bottom of files
- Use `#[cfg(feature = "testing")]` for tests requiring mock dependencies
- Use `tempfile::TempDir` for tests that need temporary files
- Tests should follow the `test_<function_name>_<scenario>` naming pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_handler_new() {
        let temp_dir = TempDir::new().unwrap();
        // test logic
    }

    #[cfg(feature = "testing")]
    #[test]
    fn test_with_mock() {
        let stub = StubDockerClient::new().with_list_containers(vec![]);
        // test logic
    }
}
```

### Trait Design

Use traits for abstraction points (like `DockerClient`) that may have multiple implementations:

```rust
pub trait DockerClient: Send + Sync {
    fn build_image(&self, ...) -> Result<()>;
    fn run_interactive(&self, ...) -> Result<ExitStatus>;
    fn list_containers(&self, ...) -> Result<Vec<ContainerInfo>>;
}
```

### Documentation

- Module-level documentation at the top of each file
- Public API functions should have doc comments explaining purpose
- Use doc examples for types that benefit from them

```rust
//! Docker client abstraction layer.

/// Trait for Docker operations.
/// Implement this to support different Docker backends.
pub trait DockerClient: Send + Sync {
    /// Build a Docker image.
    fn build_image(&self, ...) -> Result<()>;
}
```

### Code to Avoid

- Don't use `unwrap()` in production code (use `?` or proper error handling)
- Don't use `unsafe` blocks
- Don't introduce `println!` for debugging (use `eprintln!` or logging)
- Don't add unnecessary type annotations when type inference is clear
- Don't create deep hierarchies; prefer flat module structures
