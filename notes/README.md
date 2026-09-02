# Rust Learning Notes

Quick reference for concepts I'm learning while building with Rust. Grouped by topic — open any file for details.

## Cargo Basics

- [cargo-init.md](./cargo-init.md) — Create a Cargo project in an existing directory. Covers `cargo init`, common options (name, edition, VCS), and what gets created.
- [cargo-new.md](./cargo-new.md) — Create a new Cargo project from scratch with `cargo new`. Same options as `init`, plus the binary-vs-library choice.
- [cargo-comparison.md](./cargo-comparison.md) — `cargo init` vs `cargo new`, side by side: when to use each, identical features, and the resulting structure.

## Project Structure

- [binary-vs-library.md](./binary-vs-library.md) — The difference between binary and library crates: creating each, structure, running, use cases, and the hybrid pattern.
- [rust-project-organization.md](./rust-project-organization.md) — How to organize a Rust codebase: single binary/library, hybrid, module patterns, workspaces, and testing structure.
- [target-folder.md](./target-folder.md) — What's inside `target/`: debug vs release builds, key components, common commands.

## Language Concepts

- [rust-tools-vs-python-tools.md](./rust-tools-vs-python-tools.md) — Comparison of everyday tooling: runtime, package manager, formatter, linter, type checker, test runner, docs, REPL.
- [rust-for-python-programmers.md](./rust-for-python-programmers.md) — Side-by-side comparison of typing, memory, error handling, concurrency, and tooling when moving from Python to Rust.
- [concurrency-gil.md](./concurrency-gil.md) — Why Python's GIL caps CPU-bound parallelism, and how Rust's ownership + `Send`/`Sync` give true parallelism.
- [rust-ownership.md](./rust-ownership.md) — Ownership rules, why they prevent circular references, and when `Rc`/`Weak` are needed.
- [rust-data-types.md](./rust-data-types.md) — The type system: scalar and compound types, strings, collections, references, functions, closures, `Option`, `Result`, custom types, and smart pointers.
