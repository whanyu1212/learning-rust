# Target Folder Structure

> **TL;DR** — target/ is Cargo's scratch space: debug/ for fast dev builds, release/ for optimized output. Safe to delete, never to commit.

The `target/` folder is where Cargo stores all build artifacts and compilation outputs.

## Top-level Structure

```
target/
├── .rustc_info.json    # Cache of rustc compiler information
├── CACHEDIR.TAG        # Marks this as a cache directory
├── debug/              # Debug build artifacts (default)
└── release/            # Release build artifacts (--release flag)
```

### Top-level Files

- **`.rustc_info.json`** - Cached compiler information to avoid re-querying rustc
- **`CACHEDIR.TAG`** - Signals to backup tools that this is a cache directory and can be skipped

## Inside target/debug/ (or target/release/)

```
debug/
├── <binary-name>       # Your compiled executable
├── <binary-name>.d     # Dependency tracking file
├── deps/               # Compiled dependencies and object files
├── .fingerprint/       # Incremental compilation metadata
├── build/              # Build script outputs
├── incremental/        # Incremental compilation cache
├── examples/           # Compiled example binaries
└── .cargo-lock         # Internal build lock file
```

### Key Components

**`<binary-name>`** (executable)
- Your compiled binary, ready to run
- In debug mode: unoptimized, includes debug symbols
- In release mode: fully optimized, stripped debug info

**`<binary-name>.d`**
- Lists source files that affect the binary
- Used to determine when rebuilding is needed

**`deps/`**
- Compiled dependencies (.rlib, .so, .dylib files)
- Intermediate object files
- All external crates your project depends on

**`.fingerprint/`**
- Timestamps and content hashes of source files
- Determines what needs recompiling (incremental compilation)
- Speeds up subsequent builds

**`build/`**
- Output from build scripts (build.rs)
- Generated code and build artifacts
- Only present if dependencies use build scripts

**`incremental/`**
- Incremental compilation cache
- Stores compilation state between builds
- Significantly speeds up rebuilds

**`examples/`**
- Compiled binaries from your `examples/` folder
- Only created if you have examples

## Debug vs Release

| Aspect | debug/ | release/ |
|--------|--------|----------|
| Command | `cargo build` | `cargo build --release` |
| Optimization | None (O0) | Full (O3) |
| Compile time | Fast | Slow |
| Binary size | Larger | Smaller |
| Performance | Slower | Much faster |
| Debug symbols | Included | Stripped |
| Use case | Development | Production |

## Important Notes

> [!WARNING]
> Never commit `target/` — it can grow to hundreds of MB and is fully regenerable. Keep `/target` in `.gitignore`.

- **Safe to delete** - The entire `target/` folder can be deleted and regenerated
- **Large size** - Can grow to hundreds of MB with many dependencies
- **Build profiles** - You can also have custom profiles like `target/test/`, `target/bench/`

## Common Commands

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run debug binary directly
./target/debug/project-name

# Run release binary directly
./target/release/project-name

# Clean all build artifacts
cargo clean
```

## Key takeaways

1. `target/debug/` = fast unoptimized dev builds; `target/release/` = slow optimized production builds.
2. `deps/`, `.fingerprint/`, and `incremental/` exist to make rebuilds fast — you can ignore them day to day.
3. Delete `target/` freely when disk is tight; `cargo build` regenerates it.

## Next

- [Rust for Python Programmers](./rust-for-python-programmers.md) — from Cargo mechanics to language concepts.
