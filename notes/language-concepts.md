# Language Concepts

> **TL;DR** — Rust moves Python's runtime answers to compile time: types, memory, errors, and thread safety are checked before the program runs.

The ideas that feel different when you arrive from Python, in the order they usually click.

- [Rust for Python Programmers](./rust-for-python-programmers.md) — typing, memory, errors, and concurrency side by side.
- [Tools: Rust vs Python](./rust-tools-vs-python-tools.md) — one tool (`cargo`) vs the usual Python toolkit.
- [Data Types](./data-types.md) — scalars, collections, `Option`/`Result`, and custom types.
- [Ownership](./rust-ownership.md) — one owner per value; why cycles are hard by default.
- [Concurrency without a GIL](./concurrency-gil.md) — true parallelism via `Send`/`Sync`, not a global lock.
