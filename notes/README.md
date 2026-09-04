# Rust Learning Notes

Quick reference for concepts I'm learning while moving from Python to Rust. Grouped by topic — open any chapter for details.

## Cargo Basics

- [cargo init](./cargo-init.md) — initialize a Cargo package in an existing directory.
- [cargo new](./cargo-new.md) — create a new Cargo package (directory included).
- [Init vs New](./cargo-comparison.md) — side by side: when to use each, the shared option reference, and the resulting structure.

## Project Structure

- [Binary vs Library](./binary-vs-library.md) — executable programs vs reusable crates: creating each, running, use cases, and the hybrid pattern.
- [Project Organization](./rust-project-organization.md) — how to lay out a codebase: single binary/library, hybrid, module patterns, workspaces, and tests.
- [The target/ Folder](./target-folder.md) — what's inside `target/`: debug vs release builds, key components, common commands.

## Language Concepts

- [Rust for Python Programmers](./rust-for-python-programmers.md) — typing, memory, error handling, and concurrency compared side by side.
- [Tools: Rust vs Python](./rust-tools-vs-python-tools.md) — everyday tooling: runtime, package manager, formatter, linter, type checker, tests, docs, REPL.
- [Data Types: Scalars & Strings](./rust-data-types.md) — integers, floats, bools, chars, tuples, arrays, and `String` vs `&str`.
- [Data Types: Collections & Refs](./rust-collections.md) — `Vec`, `HashMap`, `HashSet`, references, functions, and closures.
- [Data Types: Option, Result & Custom](./rust-custom-types.md) — `Option`, `Result`, structs, enums, and smart pointers.
- [Ownership](./rust-ownership.md) — ownership rules, why they prevent circular references, and when `Rc`/`Weak` are needed.
- [Concurrency without a GIL](./concurrency-gil.md) — why Python's GIL caps CPU-bound parallelism, and how Rust's ownership + `Send`/`Sync` give true parallelism.
