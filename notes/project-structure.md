# Project Structure

> **TL;DR** — `src/main.rs` is a program you run; `src/lib.rs` is code others import. Layout lives directly under `src/`; build output goes in `target/`.

How a crate is laid out on disk, and what Cargo generates when you build.

- [Binary vs Library](./binary-vs-library.md) — executables vs reusable crates, and the hybrid pattern.
- [Project Organization](./rust-project-organization.md) — modules, workspaces, tests, and when to split.
- [The target/ Folder](./target-folder.md) — debug vs release artifacts; safe to delete, never commit.
