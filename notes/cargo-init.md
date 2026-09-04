# cargo init

> **TL;DR** — Run `cargo init` when you're already inside a directory and want to turn it into a Rust package.

Initialize a Cargo package in an **existing directory**.

> [!NOTE]
> `cargo init` and `cargo new` accept the same options. For the full option reference, see [Init vs New](./cargo-comparison.md).

## Basic Usage

```bash
mkdir my_project && cd my_project
cargo init
```

Creates a binary package in the current directory by default. To initialize a different directory instead, pass its path (defaults to `.`):

```bash
cargo init path/to/directory
```

## Binary or Library

```bash
cargo init          # binary (default)
cargo init --bin    # binary, explicit
cargo init --lib    # library
```

## Examples

```bash
# Binary in the current directory
cargo init

# Library with a custom package name
cargo init --lib --name my_library

# 2024 edition binary, no version control
cargo init --edition 2024 --vcs none
```

## What Gets Created

- `Cargo.toml` — package manifest
- `src/main.rs` — entry point (binary) or `src/lib.rs` (library)
- `.gitignore` — ignore file (if VCS is enabled)
- `.git/` — repository (if `--vcs git` and the directory isn't already a repo)

## See also

- [cargo new](./cargo-new.md) — create a project *and* its directory in one command
- [Init vs New](./cargo-comparison.md) — full option reference and side-by-side workflows

## Try it

```bash
mkdir try-init && cd try-init
cargo init
cargo run
```
