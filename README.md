# 🔍 VSCWhere

A command-line tool to locate Visual Studio Code installations on Windows, inspired by [vswhere](https://github.com/microsoft/vswhere).

[![Platform](https://img.shields.io/badge/platform-Windows-blue?style=for-the-badge)]()
[![Rust](https://img.shields.io/badge/rust-stable-orange?style=for-the-badge&logo=rust)]()
[![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)]()
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow?style=for-the-badge)](https://conventionalcommits.org)
[![Build](https://img.shields.io/github/actions/workflow/status/CodingWithCalvin/VSCWhere/build.yml?style=for-the-badge&label=Build)](https://github.com/CodingWithCalvin/VSCWhere/actions)

## 🤔 Why VSCWhere?

Need to find VS Code installations in scripts, CI/CD pipelines, or tooling? VSCWhere discovers all VS Code installations on your system and returns their paths, versions, and metadata - just like `vswhere.exe` does for Visual Studio.

### Key Features

✅ **Discover All Installations**: Finds Stable, Insiders, and Exploration builds

✅ **vswhere-Compatible CLI**: Familiar flags like `-format`, `-property`, `-latest`

✅ **JSON Output**: Easy to parse in scripts and automation

✅ **Tiny Binary**: ~220 KB standalone executable, no dependencies

## 📦 Installation

Download `vscwhere.exe` from the [latest release](https://github.com/CodingWithCalvin/VSCWhere/releases/latest) and place it in your PATH.

## 🚀 Usage

```bash
# List all VS Code installations
vscwhere

# Include Insiders builds
vscwhere -prerelease

# Get only the latest version
vscwhere -latest

# Output as JSON
vscwhere -format json

# Get just the installation path
vscwhere -property installationPath

# Combine flags
vscwhere -latest -format json -nologo
```

### Example Output

```
VSCWhere version 0.1.0

installationPath: C:\Users\user\AppData\Local\Programs\Microsoft VS Code\
installationVersion: 1.107.1
productPath: C:\Users\user\AppData\Local\Programs\Microsoft VS Code\Code.exe
productId: stable
isPrerelease: false
displayName: Visual Studio Code
extensionsPath: C:\Users\user\.vscode\extensions
userDataPath: C:\Users\user\AppData\Roaming\Code
```

### JSON Output

```json
[
  {
    "installationPath": "C:\\Users\\user\\AppData\\Local\\Programs\\Microsoft VS Code\\",
    "installationVersion": "1.107.1",
    "productPath": "C:\\Users\\user\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe",
    "productId": "stable",
    "isPrerelease": false,
    "displayName": "Visual Studio Code",
    "extensionsPath": "C:\\Users\\user\\.vscode\\extensions",
    "userDataPath": "C:\\Users\\user\\AppData\\Roaming\\Code"
  }
]
```

## 📋 CLI Reference

| Flag | Description |
|------|-------------|
| `-all` | Find all instances (default) |
| `-prerelease` | Include Insiders/prerelease builds |
| `-latest` | Return only the latest version |
| `-format <type>` | Output format: `text` (default), `json` |
| `-property <name>` | Return only the specified property |
| `-nologo` | Suppress version banner |
| `-sort` | Sort by version (descending) |
| `-help`, `-?` | Show help |

### Properties

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

## 🔧 Building from Source

```bash
# Clone the repository
git clone https://github.com/CodingWithCalvin/VSCWhere.git
cd VSCWhere

# Build release binary
cargo build --release

# Binary is at target/release/vscwhere.exe
```

## 👥 Contributors

<!-- readme: contributors -start -->
<!-- readme: contributors -end -->

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

Made with ❤️ by [Coding With Calvin](https://github.com/CodingWithCalvin)
