# CLAUDE.md

This file provides guidance to Claude Code when working with code in this repository.

## Critical Rules

1. **NEVER commit directly to main** - Always create a feature branch and submit a pull request
2. **Conventional commits** - Format: `type(scope): description`
3. **GitHub Issues for TODOs** - Use `gh` CLI to manage issues, no local TODO files
4. **Pull Request titles** - Use conventional commit format
5. **Branch naming** - Use format: `type/scope/short-description`

## Project Overview

VSCWhere is a CLI tool that locates Visual Studio Code installations on Windows, similar to how `vswhere.exe` works for Visual Studio.

### Purpose

- Find all VS Code installations (Stable, Insiders, Exploration)
- Return installation metadata (path, version, product info)
- Support multiple output formats (text, JSON)
- Provide vswhere-compatible command-line interface

## Tech Stack

- Rust (latest stable)
- Windows-only (Registry-based discovery)

## Build Commands

```bash
# Build debug
cargo build

# Build release (optimized for size)
cargo build --release

# Run
cargo run -- -help

# Run release
cargo run --release
```

## Project Structure

```
VSCWhere/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point and CLI parsing
│   ├── discovery.rs     # Registry discovery logic
│   ├── models.rs        # Data structures
│   └── output.rs        # Text/JSON formatting
├── .github/
│   └── workflows/       # CI/CD
└── CLAUDE.md
```

## CLI Interface

Mirrors vswhere.exe flags where applicable:

| Flag | Description |
|------|-------------|
| `-all` | Find all instances (default behavior) |
| `-prerelease` | Include Insiders/prerelease builds |
| `-latest` | Return only the latest version |
| `-format [text\|json]` | Output format (default: text) |
| `-property <name>` | Output specific property only |
| `-nologo` | Suppress version banner |
| `-sort` | Sort results by version |
| `-utf8` | Output using UTF-8 encoding |
| `-help`, `-?` | Show help |

## Discovery Strategy

1. Query Windows Registry uninstall entries for Microsoft VS Code installations
2. Read `InstallLocation` from registry entries
3. Parse `resources/app/package.json` and `product.json` for version/product info
4. Derive extensions and user data paths from product type

## Output Properties

| Property | Description |
|----------|-------------|
| `installationPath` | VS Code installation directory |
| `installationVersion` | Version number |
| `productPath` | Path to Code.exe |
| `productId` | Product identifier (stable/insider) |
| `isPrerelease` | True for Insiders builds |
| `displayName` | Human-readable product name |
| `extensionsPath` | User extensions directory |
| `userDataPath` | User settings/data directory |
