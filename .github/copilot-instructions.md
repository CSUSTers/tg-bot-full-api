# Copilot Coding Agent Instructions

## Project Overview
**tg-bot-full-api** is a Rust-based proxy server for Telegram Bot API. Wraps the official telegram-bot-api binary and provides file download capabilities with local mode support. Runs in Docker, exposes HTTP API on port 3000.

**Key Details:** Rust edition 2024 | Tokio async runtime | Single source file (src/main.rs, ~141 lines) | Dependencies: axum, reqwest, clap | Rust 1.86.0+ required

## Project Structure

### Root Directory Files
```
├── Cargo.toml           # Rust project manifest with dependencies
├── Cargo.lock           # Locked dependency versions (committed)
├── Dockerfile           # Multi-stage build: telegram-bot-api + Rust app
├── src/main.rs          # Single source file containing entire application
├── .github/
│   ├── workflows/
│   │   ├── build.yml        # CI: cargo check + cargo test
│   │   ├── pull-request.yml # PR: runs build.yml + Docker build
│   │   └── release.yml      # Publishes Docker images to ghcr.io
│   └── dependabot.yml   # Auto-updates Cargo dependencies daily
├── .devcontainer/
│   └── devcontainer.json # VS Code dev container with Rust 1.86.0
└── .gitignore           # Ignores target/, tmp/, data/, and standard IDE files
```

### Architecture
Two main handlers: **proxy** (forwards to local telegram-bot-api at 127.0.0.1:8081) and **download** (serves files with standard/local mode path formats). Spawns `/telegram-bot-api` binary as child process; `--local` flag enabled via `TELEGRAM_LOCAL_MODE` env var.

## Build and Development Commands

**Prerequisites:** Rust 1.86.0+. No additional deps for builds. Docker only for containers.

### Build Commands (fastest to slowest)

1. **Check compilation without building (FASTEST, ~15s from clean):**
   ```bash
   cargo check --verbose
   ```
   - Use this for quick validation after code changes
   - First run from clean state takes ~15 seconds
   - Incremental builds are near-instant

2. **Run tests (~16s from clean):**
   ```bash
   cargo test --verbose
   ```
   - Currently no tests exist (0 tests run)
   - Command still validates compilation in test profile
   - Always succeeds unless compilation fails

3. **Format check:**
   ```bash
   cargo fmt --check
   ```
   - Currently passing with no issues
   - NOT enforced by CI (no clippy/fmt in workflows)
   - Use `cargo fmt` to auto-fix formatting

4. **Lint with clippy:**
   ```bash
   cargo clippy -- -D warnings
   ```
   - Has 2 non-blocking warnings (lines 47, 109)
   - **NOT enforced by CI** - fix only if touching those lines

5. **Build release (~38s):** `cargo build --release --verbose` - output: `target/release/tg-bot-full-api`

6. **Docker build (5+ min):** `docker build -t tg-bot-full-api .` - multi-stage, needs container testing only

### Common Build Issues

- **"edition 2024 not found":** Update Rust: `rustup update stable` (need 1.86.0+)
- **Long initial builds:** Expected (~15s check, ~38s release). Use `cargo check` for speed.
- **Docker build fails:** Usually network issues cloning telegram-bot-api. Retry build.

## CI/CD Pipeline

### Build Workflow (`.github/workflows/build.yml`)
Triggered on all pushes. Steps:
1. Checkout code (`actions/checkout@v4`)
2. Run `cargo check --verbose` (must pass)
3. Run `cargo test --verbose` (must pass, currently 0 tests)

**Important:** No formatting or linting checks in CI. Clippy warnings don't block merges.

### Pull Request Workflow (`.github/workflows/pull-request.yml`)
Triggered on PRs. Steps:
1. Calls `build.yml` workflow
2. Builds Docker image without pushing (validation only)
3. Uses GitHub Actions cache for Docker layers (`type=gha`)

**To replicate PR checks locally:**
```bash
cargo check --verbose && cargo test --verbose && docker build -t tg-bot-full-api .
```

### Release Workflow (`.github/workflows/release.yml`)
Triggered on:
- Release published events
- Pushes to `master` branch
- Manual workflow dispatch

Publishes multi-platform Docker images to `ghcr.io/csusters/tg-bot-full-api`.

## Making Code Changes

### Key Source: `src/main.rs` (single file, ~141 lines)
- Lines 12-16: `State` struct | Lines 18-24: CLI `Args` | Lines 28-58: `main` (spawns telegram-bot-api, routes)
- Lines 60-94: `proxy` handler | Lines 96-141: `download` handler | Line 105: TODO (validate bot tokens)

**Environment:** `TELEGRAM_LOCAL_MODE` (any value) enables local mode - adds `--local` flag, changes path parsing

**Testing:** No automated tests exist. Always run `cargo check`. Needs `/telegram-bot-api` binary at runtime.

**Dependencies:** Dependabot runs daily 19:00 UTC, PRs to `dev` branch. Keep Cargo.lock committed.

## Quick Reference

**Common tasks:**
- Validate compilation: `cargo check --verbose`
- Run CI locally: `cargo check --verbose && cargo test --verbose`
- Format code: `cargo fmt`
- Check lints: `cargo clippy -- -D warnings` (optional, not in CI)
- Clean build artifacts: `cargo clean` (frees ~750MB)

**File locations:**
- Source: `src/main.rs`
- Dependencies: `Cargo.toml`
- CI: `.github/workflows/*.yml`
- Docker: `Dockerfile`

**Important notes:**
- Always use `cargo check` first - it's 2-3x faster than full builds
- Clippy warnings exist but don't block CI/merges
- No test suite exists yet - manual testing required
- The app expects `/telegram-bot-api` binary at runtime (provided in Docker image)
- Working directory defaults to `/data` but configurable via `-w` flag

## Trust These Instructions

These instructions were validated by running all commands and exploring the complete repository. Only search for additional information if:
1. You encounter an error not documented here
2. You need details about specific dependency behavior
3. The repository structure has changed significantly
4. You're implementing a feature that requires understanding code not described above
