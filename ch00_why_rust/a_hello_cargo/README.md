# Getting Started with Cargo

## What You'll Learn

How to initialize, build, and run Rust projects using Cargo - Rust's build system and package manager.

## Core Commands

### Initialize a New Project

```bash
# Create a new binary project
cargo init

# Create a new library project
cargo init --lib
```

### Build Your Project

```bash
# Development build (with debug symbols)
cargo build

# Production build (optimized)
cargo build --release
```

### Output Location

- Development builds: `target/debug/<project-name>`
- Release builds: `target/release/<project-name>`

## Running Your Project

```bash
# Run the project directly
cargo run

# Run with release optimizations
cargo run --release
```