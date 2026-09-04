# Init vs New

> **TL;DR** — Same scaffolding, different starting point: `init` fills the directory you're in, `new` creates one. Default to `new`.

`cargo init` and `cargo new` do the same thing — scaffold a Cargo package. The only difference is the directory: `init` uses an existing one, `new` creates it.

## Quick Reference

| Feature | `cargo init` | `cargo new` |
|---------|-------------|------------|
| **Directory** | Uses existing directory | Creates new directory |
| **Path argument** | Optional (defaults to `.`) | Required |
| **Typical use** | Already in a project folder; converting an existing project to Cargo | Starting a fresh project |
| **Options** | Same | Same |
| **Output structure** | Same | Same |

## Shared Options (Canonical Reference)

Both commands accept these flags:

| Flag | Values | Default |
|------|--------|---------|
| `--bin` / `--lib` | binary or library target | `--bin` |
| `--edition` | `2015`, `2018`, `2021`, `2024` | toolchain default |
| `--name` | custom package name (defaults to directory name) | directory name |
| `--vcs` | `git`, `hg`, `pijul`, `fossil`, `none` | `git` |
| `--registry` | registry to use | crates.io |

```bash
# Same flags, either command:
cargo init --lib --edition 2024 --name my_library --vcs none
cargo new my-lib --lib --edition 2024 --name my_library --vcs none
```

## Workflows Side by Side

Starting fresh — one command with `new`:

```bash
cargo new my_app
cd my_app
cargo run
```

Same result with `init` — three steps:

```bash
mkdir my_app
cd my_app
cargo init
cargo run
```

Use `init` when you already have a directory (e.g. you cloned a repo or unpacked a template and want to Cargo-ify it). Otherwise prefer `new` — it's one less command.

## Both Create the Same Structure

```
my_project/
├── Cargo.toml
├── .gitignore
├── .git/
└── src/
    └── main.rs  (or lib.rs for --lib)
```

## Common Pattern

Many developers default to `cargo new` because it folds directory creation into the same step:

```bash
# Three steps
mkdir project && cd project && cargo init

# One step
cargo new project && cd project
```

## Key takeaways

1. `init` fills the directory you are in; `new` creates one. Everything else is identical.
2. When in doubt, use `new` — fewer steps.
3. The shared-options table above is the canonical flag reference; the individual pages link here.

## Next

- [Binary vs Library](./binary-vs-library.md) — what the scaffolded `src/main.rs` vs `src/lib.rs` actually means.
