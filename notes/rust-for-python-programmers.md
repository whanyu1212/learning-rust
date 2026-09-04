# Rust for Python Programmers

> **TL;DR** — Rust moves Python's runtime answers to compile time: types checked before running, memory freed deterministically, None and errors as explicit types.

A side-by-side quick reference of the key differences when moving from Python to Rust.

| Concept | Python | Rust | Key Difference |
|---------|--------|------|----------------|
| **Typing** | Dynamic (duck typing) | Static (compile-time) | Errors caught before runtime |
| **Memory** | Garbage collected (ref counting + cycle GC) | Ownership system | Zero-cost, deterministic cleanup |
| **None/null** | `None` anywhere | `Option<T>` | Compile-time None safety |
| **Error handling** | `raise` / `try` / `except` | `Result<T, E>` | Explicit, no hidden control flow |
| **Mutability** | Everything mutable | Immutable by default | Opt-in to mutation |
| **Speed** | Interpreted (~10–100× slower) | Compiled (C/C++ speed) | Orders of magnitude faster |
| **Concurrency** | GIL limits threads | No GIL, `Send` / `Sync` traits | True parallelism by default |
| **Dependencies** | `pip install` / `poetry add` | `cargo add` | Built-in dependency management |
| **Build system** | setuptools/poetry/hatch | Cargo | Single unified tool |
| **Packaging** | `pyproject.toml` | `Cargo.toml` | Similar declarative config |
| **REPL** | `python` interactive | No REPL (use tests / `cargo run`) | Compile-first workflow |
| **Type hints** | Optional, not enforced | Required, compiler-enforced | Types are not decorative |

## How to read this

- **Left column** — the Python way you already know.
- **Middle column** — the Rust equivalent for the same concept.
- **Right column** — the practical consequence of the difference.

## Key takeaways

1. Rust checks at compile time what Python checks at runtime — types, borrow validity, thread safety.
2. `Option` and `Result` replace `None`-anywhere and exceptions with explicit, must-handle types.
3. Cargo replaces six-plus Python tools with one; the compiler replaces mypy entirely.

## Next

- [Tools: Rust vs Python](./rust-tools-vs-python-tools.md) — the one-tool (`cargo`) vs many-tools story behind the table rows above.
- [Data Types: Scalars & Strings](./rust-data-types.md) — start the type system from the `Typing` row.
