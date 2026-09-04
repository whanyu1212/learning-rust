# cargo new

> **TL;DR** — Run `cargo new <name>` to create a project directory and scaffold a Rust package in one step.

Create a new Cargo package at a specified path — Cargo creates the directory for you.

> [!NOTE]
> `cargo new` and `cargo init` accept the same options. For the full option reference, see [Init vs New](./cargo-comparison.md).

## Basic Usage

```bash
cargo new my_project
cd my_project
```

The path argument is required. A binary package is created by default.

## Binary or Library

```bash
cargo new my_app              # binary (default)
cargo new my_app --bin        # binary, explicit
cargo new my_lib --lib        # library
```

## Examples

```bash
# Binary app
cargo new hello_world

# Library
cargo new my_utils --lib

# 2024 edition library with a custom package name
cargo new my-lib --lib --edition 2024 --name my_library

# No version control initialization
cargo new standalone_project --vcs none
```

## What Gets Created

A new directory containing:

- `Cargo.toml` — package manifest
- `src/main.rs` — entry point (binary) or `src/lib.rs` (library)
- `.gitignore` — ignore file (if VCS is enabled)
- `.git/` — repository (if VCS is git)

## See also

- [cargo init](./cargo-init.md) — initialize a project in an *existing* directory
- [Init vs New](./cargo-comparison.md) — full option reference and side-by-side workflows

## Try it

```bash
cargo new try-new
cd try-new
cargo run
```
