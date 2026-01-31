# Binary vs Library Projects in Rust

## What's the Difference?

### Binary Project
A **binary** is an **executable program** that you can run directly.

### Library Project
A **library** is **reusable code** that other programs can import and use. It cannot run on its own.

---

## Binary Project

### Creating a Binary

```bash
cargo init          # Creates binary by default
cargo init --bin    # Explicitly create binary
cargo new my_app    # Also creates binary by default
```

### Structure

```
my_app/
├── Cargo.toml
└── src/
    └── main.rs     # Entry point with fn main()
```

### Code Example

**src/main.rs:**
```rust
fn main() {
    println!("I'm a program you can run!");
}
```

### Running

```bash
cargo run                    # Compile and run
cargo build                  # Just compile
./target/debug/my_app       # Run the compiled binary
cargo build --release        # Optimized build
./target/release/my_app     # Run optimized binary
```

### Characteristics

- ✅ Has `fn main()` function
- ✅ Can be executed with `cargo run`
- ✅ Produces a standalone executable file
- ✅ Entry point is `src/main.rs`
- ❌ Cannot be imported by other projects (by default)

### Use Cases

- Command-line tools (like `grep`, `git`, `cargo`)
- Web servers and APIs
- Desktop applications
- Games
- Scripts and automation tools
- Any standalone program

---

## Library Project

### Creating a Library

```bash
cargo init --lib        # Create library in current directory
cargo new my_lib --lib  # Create new library directory
```

### Structure

```
my_lib/
├── Cargo.toml
└── src/
    └── lib.rs      # Library entry point (no main function)
```

### Code Example

**src/lib.rs:**
```rust
// Public functions that other code can use
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Public struct
pub struct Config {
    pub name: String,
    pub version: String,
}

impl Config {
    pub fn new(name: String, version: String) -> Self {
        Config { name, version }
    }
}

// Private function (not accessible outside this crate)
fn internal_helper() {
    println!("This is private");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

### Using a Library

You cannot run a library directly. Other projects import it:

**Another project's Cargo.toml:**
```toml
[dependencies]
my_lib = { path = "../my_lib" }  # Local path
# or
my_lib = "1.0"  # From crates.io
```

**Another project's src/main.rs:**
```rust
use my_lib::{add, multiply, Config};

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);

    let config = Config::new(
        String::from("MyApp"),
        String::from("1.0.0")
    );
    println!("App: {}", config.name);
}
```

### Commands

```bash
cargo build              # Compile the library
cargo test               # Run tests
cargo doc --open         # Generate and view documentation
cargo run                # ❌ ERROR: no main function!
```

### Characteristics

- ✅ Entry point is `src/lib.rs`
- ✅ Can be imported by other projects
- ✅ Can be published to crates.io
- ❌ No `fn main()` function
- ❌ Cannot be executed directly
- ❌ Does not produce a standalone executable

### Use Cases

- Utility functions and helpers
- Framework components
- Shared business logic
- Serialization/deserialization (like `serde`)
- HTTP clients (like `reqwest`)
- Async runtimes (like `tokio`)
- Any code meant to be reused across projects

---

## Comparison Table

| Feature | Binary | Library |
|---------|--------|---------|
| **Entry file** | `src/main.rs` | `src/lib.rs` |
| **Has `fn main()`** | ✅ Required | ❌ No |
| **Run with `cargo run`** | ✅ Yes | ❌ No |
| **Produces executable** | ✅ Yes | ❌ No |
| **Can be imported** | ❌ No* | ✅ Yes |
| **Purpose** | Program to run | Code to reuse |
| **Distribution** | Binary file | Source code/crate |
| **Testing** | Integration tests | Unit + integration tests |
| **Documentation** | Less common | Very common |

*Binaries can also expose a library (see Hybrid below)

---

## Hybrid: Binary + Library

Many real-world projects have **both** a library and a binary in the same crate.

### Structure

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs      # Library code (core logic)
    └── main.rs     # Binary (CLI wrapper)
```

### Example

**src/lib.rs:**
```rust
pub fn process_data(input: &str) -> String {
    // Core business logic
    input.to_uppercase()
}

pub fn validate_input(input: &str) -> Result<(), String> {
    if input.is_empty() {
        Err(String::from("Input cannot be empty"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        assert_eq!(process_data("hello"), "HELLO");
    }
}
```

**src/main.rs:**
```rust
use my_project::{process_data, validate_input};

fn main() {
    let input = "hello world";

    match validate_input(input) {
        Ok(_) => {
            let result = process_data(input);
            println!("Result: {}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

### Benefits of Hybrid Approach

1. **Separation of concerns**: Core logic in `lib.rs`, UI/CLI in `main.rs`
2. **Reusability**: Others can use your library without the CLI
3. **Testability**: Test core logic independently
4. **Flexibility**: Provide both a library and a tool

### Real-World Examples

- **ripgrep** - Search library + CLI tool
- **cargo** - Build system library + CLI
- **tokio** - Async runtime library + utilities
- **serde** - Serialization library + derive macros

---

## Multiple Binaries

You can have multiple binary targets in one project:

### Structure

```
my_project/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Shared library code
│   ├── main.rs             # Default binary
│   └── bin/
│       ├── server.rs       # Additional binary
│       ├── client.rs       # Additional binary
│       └── admin.rs        # Additional binary
```

### Running Different Binaries

```bash
cargo run                   # Runs src/main.rs
cargo run --bin server      # Runs src/bin/server.rs
cargo run --bin client      # Runs src/bin/client.rs
cargo run --bin admin       # Runs src/bin/admin.rs
```

### Example Binary in bin/

**src/bin/server.rs:**
```rust
use my_project::process_data;  // Use library code

fn main() {
    println!("Starting server...");
    let data = process_data("server");
    println!("Processed: {}", data);
}
```

---

## Examples Directory

Both binaries and libraries can have example programs:

### Structure

```
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── examples/
    ├── basic.rs
    ├── advanced.rs
    └── demo.rs
```

### Running Examples

```bash
cargo run --example basic
cargo run --example advanced
cargo run --example demo
```

### Example File

**examples/basic.rs:**
```rust
use my_project::process_data;

fn main() {
    println!("Basic example:");
    let result = process_data("example");
    println!("Result: {}", result);
}
```

---

## When to Use Each

### Choose Binary when:
- Building a CLI tool or application
- Creating a web server
- Making a game or desktop app
- Building a script or automation tool
- The primary goal is to **run** something

### Choose Library when:
- Creating reusable utilities
- Building framework components
- Sharing code across multiple projects
- Publishing to crates.io for others to use
- The primary goal is to **provide functionality**

### Choose Hybrid (Both) when:
- Building a tool that others might want to integrate
- You want to test core logic separately from the UI
- You want to provide both a CLI and a library API
- You're building a complex application with reusable components

---

## Quick Commands Reference

### Binary Project

```bash
cargo init              # Create binary
cargo run               # Compile and run
cargo build             # Just compile
cargo build --release   # Optimized build
```

### Library Project

```bash
cargo init --lib        # Create library
cargo build             # Compile
cargo test              # Run tests
cargo doc --open        # Generate docs
cargo publish           # Publish to crates.io
```

### Hybrid Project

```bash
cargo run               # Run the binary
cargo test              # Test the library
cargo build --lib       # Build only library
cargo build --bin name  # Build specific binary
```

---

## Converting Between Types

### Binary → Add Library

Just create `src/lib.rs`:

```bash
# You already have src/main.rs
touch src/lib.rs
```

Now you have both!

### Library → Add Binary

Create `src/main.rs`:

```bash
# You already have src/lib.rs
touch src/main.rs
```

Add a `main()` function and you're done!

### Check Current Type

```bash
ls src/
# main.rs = binary
# lib.rs = library
# both = hybrid
```

---

## Summary

- **Binary** = Executable program with `src/main.rs` and `fn main()`
- **Library** = Reusable code with `src/lib.rs`, no `main()`
- **Hybrid** = Both `src/main.rs` and `src/lib.rs` in same project
- Use `cargo run` for binaries, `cargo test` for libraries
- Real-world projects often use the hybrid approach
