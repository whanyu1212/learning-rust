# Learning Rust

Just some personal notes on learning `Rust`

The notes are published as a static site via [mdBook](https://rust-lang.github.io/mdBook/) and deployed to GitHub Pages from the `main` branch. See `.github/workflows/deploy-mdbook.yml`.

## Local build

The Mermaid runtime is not committed — regenerate it before building locally:

```bash
cargo install mdbook mdbook-mermaid
mdbook-mermaid install .
mdbook build
```

For a live-reload preview:

```bash
mdbook serve
```

## Notes

The published book nests these as numbered sections (`1`, `1.1`, `1.2`, …). Introduction is unnumbered.

### [Cargo Basics](notes/cargo-basics.md)
- [cargo init](notes/cargo-init.md) — Initialize a Cargo package in an existing directory
- [cargo new](notes/cargo-new.md) — Create a new Cargo package at a specified path
- [Init vs New](notes/cargo-comparison.md) — Side-by-side comparison of the two commands

### [Project Structure](notes/project-structure.md)
- [Binary vs Library](notes/binary-vs-library.md) — Executable programs vs reusable libraries
- [Project Organization](notes/rust-project-organization.md) — How code is laid out under `src/`
- [The target/ Folder](notes/target-folder.md) — Where Cargo stores build artifacts

### [Language Concepts](notes/language-concepts.md)
- [Rust for Python Programmers](notes/rust-for-python-programmers.md) — Key differences when moving from Python
- [Tools: Rust vs Python](notes/rust-tools-vs-python-tools.md) — Everyday tooling compared
- [Data Types](notes/data-types.md)
  - [Scalars & Strings](notes/rust-data-types.md) — Scalar and compound types, `String` vs `&str`
  - [Collections & Refs](notes/rust-collections.md) — `Vec`, `HashMap`, references, closures
  - [Option, Result & Custom](notes/rust-custom-types.md) — Null-safety, error handling, smart pointers
- [Ownership](notes/rust-ownership.md) — Ownership rules and circular references
- [Concurrency without a GIL](notes/concurrency-gil.md) — Python's GIL vs Rust parallelism
