# Cargo New

Create a new Cargo package at a specified path (creates a new directory).

## Basic Usage

```bash
cargo new my_project
```

Creates a new directory called `my_project` with a Rust project inside.

## Common Options

### Binary vs Library

```bash
# Create a binary (application) - this is the default
cargo new my_app --bin

# Create a library
cargo new my_lib --lib
```

### Set Package Name

```bash
# Set custom package name (different from directory name)
cargo new my-directory --name my_package_name
```

### Specify Rust Edition

```bash
# Use a specific Rust edition
cargo new my_project --edition 2021
cargo new my_project --edition 2024

# Available editions: 2015, 2018, 2021, 2024
```

### Version Control

```bash
# Initialize with git (default)
cargo new my_project --vcs git

# Initialize with other VCS
cargo new my_project --vcs hg      # Mercurial
cargo new my_project --vcs pijul   # Pijul
cargo new my_project --vcs fossil  # Fossil

# Don't initialize any VCS
cargo new my_project --vcs none
```

## Examples

```bash
# Create a binary app
cargo new hello_world

# Create a library
cargo new my_utils --lib

# Create a 2024 edition library with custom name
cargo new my-lib --lib --edition 2024 --name my_library

# Create without git initialization
cargo new standalone_project --vcs none
```

## What Gets Created

A new directory with:
- `Cargo.toml` - Package manifest
- `src/main.rs` - Entry point for binary (if `--bin`)
- `src/lib.rs` - Entry point for library (if `--lib`)
- `.gitignore` - Git ignore file (if VCS is enabled)
- `.git/` - Git repository (if VCS is git)

## Key Difference from `cargo init`

`cargo new` creates a new directory for the project, while `cargo init` initializes a project in an existing directory.
