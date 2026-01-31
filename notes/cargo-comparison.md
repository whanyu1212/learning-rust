# Cargo Init vs Cargo New - Comparison

## Quick Reference

| Feature | `cargo init` | `cargo new` |
|---------|-------------|------------|
| **Directory** | Uses existing directory | Creates new directory |
| **Typical Use** | Already in project folder | Starting fresh project |
| **Path Argument** | Optional (defaults to `.`) | Required |
| **Options** | Same as `cargo new` | Same as `cargo init` |

## When to Use Each

### Use `cargo init` when:

- You already have a directory and want to initialize a Rust project in it
- You're converting an existing project to use Cargo
- You've already created and named your project directory

```bash
mkdir my_project
cd my_project
cargo init
```

### Use `cargo new` when:

- Starting a brand new project from scratch
- You want Cargo to create the project directory for you
- You're creating multiple projects and want them organized

```bash
cargo new my_project
cd my_project
```

## Example Workflow Comparison

### Workflow with `cargo new`

```bash
cargo new my_app
cd my_app
cargo run
```

### Workflow with `cargo init`

```bash
mkdir my_app
cd my_app
cargo init
cargo run
```

## Identical Features

Both commands support the same options:

- `--bin` / `--lib` - Choose between binary or library
- `--edition` - Specify Rust edition (2015, 2018, 2021, 2024)
- `--name` - Set custom package name
- `--vcs` - Choose version control system (git, hg, pijul, fossil, none)
- `--registry` - Specify registry to use

## Both Create the Same Structure

Whether you use `init` or `new`, you get:

```
my_project/
├── Cargo.toml
├── .gitignore
├── .git/
└── src/
    └── main.rs  (or lib.rs for --lib)
```

## Common Pattern

Many developers prefer `cargo new` because it's one less command:

```bash
# Two commands
mkdir project && cd project && cargo init

# One command
cargo new project && cd project
```
