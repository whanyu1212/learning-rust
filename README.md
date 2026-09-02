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

### Project Setup
- [cargo init](notes/cargo-init.md) — Initialize a Cargo package in an existing directory
- [cargo new](notes/cargo-new.md) — Create a new Cargo package at a specified path
- [cargo init vs cargo new](notes/cargo-comparison.md) — Side-by-side comparison of the two commands

### Project Structure
- [Binary vs Library Projects](notes/binary-vs-library.md) — Executable programs vs reusable libraries
- [Rust Project Organization](notes/rust-project-organization.md) — How code is laid out under `src/`
- [Target Folder Structure](notes/target-folder.md) — Where Cargo stores build artifacts

### Rust Concepts
- [Rust Data Types](notes/rust-data-types.md) — Scalar and compound types, type inference
