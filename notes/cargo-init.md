# Cargo Init

Initialize a new Cargo package in an existing directory.

## Basic Usage

```bash
cargo init
```

Creates a new Rust project in the current directory with default settings (binary application).

## Common Options

### Binary vs Library

```bash
# Create a binary (application) - this is the default
cargo init --bin

# Create a library
cargo init --lib
```

### Specify Path

```bash
# Initialize in a specific directory
cargo init path/to/directory
```

### Set Package Name

```bash
# Set custom package name (defaults to directory name)
cargo init --name my_custom_name
```

### Specify Rust Edition

```bash
# Use a specific Rust edition
cargo init --edition 2021
cargo init --edition 2024

# Available editions: 2015, 2018, 2021, 2024
```

### Version Control

```bash
# Initialize with git (default)
cargo init --vcs git

# Initialize with other VCS
cargo init --vcs hg      # Mercurial
cargo init --vcs pijul   # Pijul
cargo init --vcs fossil  # Fossil

# Don't initialize any VCS
cargo init --vcs none
```

## Examples

```bash
# Create a binary app in current directory
cargo init

# Create a library with custom name
cargo init --lib --name my_library

# Create a 2024 edition binary without git
cargo init --edition 2024 --vcs none

# Initialize in specific directory with custom name
cargo init my_project --name awesome_project
```

## What Gets Created

- `Cargo.toml` - Package manifest
- `src/main.rs` - Entry point for binary (if `--bin`)
- `src/lib.rs` - Entry point for library (if `--lib`)
- `.gitignore` - Git ignore file (if VCS is enabled)

## Key Difference from `cargo new`

`cargo init` works in an existing directory, while `cargo new` creates a new directory for the project.
