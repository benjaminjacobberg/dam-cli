⚠️ **Work in Progress** — This tool is actively being developed. Things may break. PRs welcome!

# DAM: Desperately Avoiding Maliciousness

*A Docker wrapper that keeps your machine safe from the wild world of running AI-generated code.*

## Wait, What?

You want to run [OpenCode](https://opencode.ai) locally? That's cool! But here's the thing: AI-generated code can be... let's say *ambitious*. It might try to `rm -rf /`, connect to sketchy servers, or eat all your RAM like it's at an all-you-can-eat buffet.

**DAM** wraps OpenCode in a nice, cozy Docker container so all that chaos stays contained. Think of it as a bouncer for your operating system.

## Prerequisites

- [Docker](https://docker.com) or [Podman](https://podman.io) installed and running
- Rust toolchain (for building from source)

## Installation

```bash
cargo install --path .
```

## Shell Alias

Add this to your `~/.zshrc` (or `~/.bashrc`):

```bash
alias dam='~/Workspace/prototype/dam/dam-cli/target/debug/dam'
```

Then reload your shell:

```bash
source ~/.zshrc
```

Or just run directly:

```bash
cargo run -- --help
```

## Commands

| Command        | Description                                                  |
|----------------|--------------------------------------------------------------|
| `dam build`    | Build the Docker image (pass `--stack java` or other stacks) |
| `dam run`      | Fire up an interactive container                             |
| `dam acp`      | Run as an ACP server for Zed/IntelliJ integration            |
| `dam status`   | See who's currently running                                  |
| `dam teardown` | Clean up all the containers                                  |
| `dam web`      | Launch OpenCode web application                              |
| `dam debug`    | Peek behind the curtain                                      |

## Quick Start

```bash
# Build the image
dam build --stack java

# Run it!
dam run

# Done? Clean up
dam teardown
```
