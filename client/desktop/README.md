# ScrDesk Desktop Client

Cross-platform desktop client for Windows, macOS, and Linux.

## Build Instructions

### Prerequisites
- Rust 1.75+
- Node.js 20+ (for UI)
- Platform-specific tools:
  - Windows: Visual Studio Build Tools
  - macOS: Xcode Command Line Tools
  - Linux: build-essential, libgtk-3-dev

### Build Commands

**Windows:**
```bash
cargo build --release --target x86_64-pc-windows-msvc
```

**macOS (Intel):**
```bash
cargo build --release --target x86_64-apple-darwin
```

**macOS (Apple Silicon):**
```bash
cargo build --release --target aarch64-apple-darwin
```

**Linux:**
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

## Installation

Download the latest release from:
https://github.com/shosgoren/scrdesk/releases/latest

Or visit: https://scrdesk.com/downloads
