# AGENTS.md

General repository guidance for contributors and automated agents working on `tg-bot-full-api`.

## Project Overview

`tg-bot-full-api` is a Rust-based proxy server for the Telegram Bot API. It wraps the official `telegram-bot-api` binary and adds file download support with local mode compatibility. The server runs in Docker and exposes an HTTP API on port 3000.

**Key details:** Rust edition 2024 | Tokio async runtime | Axum HTTP server | Dependencies include `axum`, `axum-extra`, `reqwest`, and `clap` | Rust 1.86.0+ required

## Project Structure

### Root Directory Files

```text
├── AGENTS.md            # General repository guidance for contributors and agents
├── Cargo.toml           # Rust project manifest with dependencies
├── Cargo.lock           # Locked dependency versions (committed)
├── Dockerfile           # Multi-stage build: telegram-bot-api + Rust app
├── src/main.rs          # Single source file containing the application
├── .github/
│   ├── workflows/
│   │   ├── build.yml        # CI: cargo check + cargo test
│   │   ├── pull-request.yml # PR: runs build.yml + Docker build
│   │   └── release.yml      # Publishes Docker images to ghcr.io
│   └── dependabot.yml   # Auto-updates Cargo dependencies
├── .devcontainer/
│   └── devcontainer.json # VS Code dev container with Rust 1.86.0
└── .gitignore           # Ignores target/, tmp/, data/, and standard IDE files
```

## Architecture

The application has two primary handlers:

- **proxy**: forwards API requests to the local `telegram-bot-api` process at `127.0.0.1:8081`.
- **download**: serves files using standard and local-mode Telegram Bot API path formats.

At runtime, the server spawns `/telegram-bot-api` as a child process. Setting the `TELEGRAM_LOCAL_MODE` environment variable adds the `--local` flag and changes file path parsing behavior.

## Build and Development Commands

**Prerequisites:** Rust 1.86.0+. Docker is only required for container builds.

### Build Commands

1. **Check compilation without building:**

   ```bash
   cargo check --verbose
   ```

   Use this for quick validation after code changes.

2. **Run tests:**

   ```bash
   cargo test --verbose
   ```

   The repository currently has no automated tests, but this still validates compilation in the test profile.

3. **Check formatting:**

   ```bash
   cargo fmt --check
   ```

   Formatting is not enforced by CI. Use `cargo fmt` to apply standard Rust formatting.

4. **Run clippy manually:**

   ```bash
   cargo clippy -- -D warnings
   ```

   Clippy is not enforced by CI. Existing non-blocking warnings may need to be addressed before this command succeeds with `-D warnings`.

5. **Build release binary:**

   ```bash
   cargo build --release --verbose
   ```

   The release output is `target/release/tg-bot-full-api`.

6. **Build Docker image:**

   ```bash
   docker build -t tg-bot-full-api .
   ```

   Docker builds are slower because the image includes the official `telegram-bot-api` binary.

### Common Build Issues

- **`edition 2024` not found:** update Rust to 1.86.0 or newer.
- **Long initial builds:** expected for clean builds; use `cargo check` for faster iteration.
- **Docker build fails while fetching `telegram-bot-api`:** retry, as this is usually a transient network issue.

## CI/CD Pipeline

### Build Workflow (`.github/workflows/build.yml`)

Triggered on pushes and reusable workflow calls:

1. Checkout code with `actions/checkout@v4`.
2. Run `cargo check --verbose`.
3. Run `cargo test --verbose`.

Formatting and clippy are not enforced by CI.

### Pull Request Workflow (`.github/workflows/pull-request.yml`)

Triggered on pull requests:

1. Runs the build workflow.
2. Builds the Docker image without pushing it.
3. Uses GitHub Actions cache for Docker layers.

To replicate PR checks locally:

```bash
cargo check --verbose && cargo test --verbose && docker build -t tg-bot-full-api .
```

### Release Workflow (`.github/workflows/release.yml`)

Triggered by release publication, pushes to `master`, and manual workflow dispatch. Publishes Docker images to `ghcr.io/csusters/tg-bot-full-api`.

## Making Code Changes

`src/main.rs` contains the application:

- `State` stores the working directory and local Telegram Bot API URL.
- `Args` defines CLI arguments; the working directory defaults to `/data`.
- `main` creates the working directory, starts `/telegram-bot-api`, and registers routes.
- `proxy` forwards incoming requests to the local Telegram Bot API process.
- `download` resolves and serves file paths for standard and local mode.

Use `cargo check --verbose` first for code changes, then run targeted tests or `cargo test --verbose` as appropriate. Keep `Cargo.lock` committed when dependencies change.

## Quick Reference

- Validate compilation: `cargo check --verbose`
- Run CI checks locally: `cargo check --verbose && cargo test --verbose`
- Format code: `cargo fmt`
- Optional lint: `cargo clippy -- -D warnings`
- Clean build artifacts: `cargo clean`

## Runtime Notes

- The app expects `/telegram-bot-api` to exist at runtime; the Docker image provides it.
- The default working directory is `/data`, configurable with the `-w` or `--work-dir` flag.
- Set `TELEGRAM_LOCAL_MODE` to enable local mode behavior.

Use this file as the source of repository-specific guidance. Re-check the repository when commands fail, dependencies change, or the project structure differs from this document.
