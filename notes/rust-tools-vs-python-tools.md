# Rust Tools vs Python Tools

> **TL;DR** — One tool (cargo) replaces pip + black + flake8 + mypy + pytest + sphinx. Type checking and formatting come built in, not bolted on.

A side-by-side comparison of the everyday tooling when moving from Python to Rust.

| Purpose | Python | Rust |
|---------|--------|------|
| **Language runtime** | `python` (interpreter) | `rustc` (compiler, rarely called directly) |
| **Package manager** | pip / poetry / uv | cargo (built-in) |
| **Project config** | `pyproject.toml` | `Cargo.toml` |
| **Lock file** | `poetry.lock` / `requirements.txt` | `Cargo.lock` |
| **Virtual env** | venv / conda | Not needed (deps are per-project) |
| **Formatter** | black / ruff format | rustfmt (built-in: `cargo fmt`) |
| **Linter** | ruff / flake8 / pylint | clippy (built-in: `cargo clippy`) |
| **Type checker** | mypy / pyright | Built into compiler (always on) |
| **Test runner** | pytest | cargo test (built-in) |
| **Docs** | sphinx / mkdocs | cargo doc (built-in) |
| **REPL** | python / ipython | None (use `cargo test` or Rust Playground) |

## The big theme

Rust's tooling is **unified into Cargo** — one tool handles package management, builds, tests, docs, formatting, and linting. Python splits these across a patchwork of third-party tools (pip, poetry, black, ruff, mypy, pytest, sphinx...).

| Rust | Python equivalent(s) |
|------|---------------------|
| `cargo add` | pip install / poetry add |
| `cargo build` | (no direct equivalent — interpret + run) |
| `cargo run` | python script.py |
| `cargo test` | pytest |
| `cargo fmt` | black / ruff format |
| `cargo clippy` | ruff / flake8 / pylint |
| `cargo doc` | sphinx / mkdocs |
| `cargo check` | mypy (catches type errors without running) |

## Key takeaways

1. **Type checking is free.** Python needs a separate `mypy`/`pyright` step; Rust's compiler does it always. No extra tool to install.
2. **No virtual env needed.** Cargo scopes dependencies per-project in `Cargo.lock` — no `venv`/`conda` activation dance.
3. **One tool, not many.** Learning `cargo` replaces learning pip + black + flake8 + mypy + pytest + sphinx.
4. **No REPL.** The Rust workflow is compile-and-run (or `cargo test`); for quick experiments, use the Rust Playground.

## Next

- [Data Types: Scalars & Strings](./rust-data-types.md) — meet the types the compiler is checking for you.
