# Data Types: Option, Result, Custom Types & Smart Pointers

> **TL;DR** — `Option<T>` replaces `None`-anywhere with compile-time null safety. `Result<T, E>` replaces `raise`/`try` with explicit error values. `struct`/`enum` replace `dataclass`/`Enum`, and smart pointers (`Box`, `Rc`, `Arc`) make heap ownership explicit.

## Option: Null Safety

```rust
let some_number = Some(5);
let absent_number: Option<i32> = None;
```

```python
some_number = 5
absent_number = None
```

The difference: in Python any variable can be `None`; in Rust only `Option<T>` can be absent, and you must handle both cases before use:

| Operation | What it does |
|-----------|--------------|
| `unwrap` / `expect` | Get the value or panic — quick prototypes only. See [Ownership](./rust-ownership.md#unwrap-pulling-a-value-out-of-result--option) |
| `unwrap_or(default)` | Value or fallback |
| `map` / `and_then` | Transform without unwrapping |
| `match` | Exhaustive handling of `Some` / `None` |

## Result: Error Handling

```rust
let ok: Result<i32, String> = Ok(42);
let err: Result<i32, String> = Err(String::from("oops"));
```

```python
# Python equivalent uses exceptions:
# return 42  vs  raise ValueError("oops")
```

| Operation | What it does |
|-----------|--------------|
| `unwrap` / `expect` | Value or panic |
| `map` / `map_err` | Transform the `Ok` / `Err` side |
| `and_then` | Chain fallible operations |
| `?` operator | Early-return the error — the ergonomic replacement for `try`/`except` propagation |

> [!NOTE]
> `?` has no Python equivalent in brevity: `let v = may_fail()?;` propagates the error to the caller in one character. Python needs a full `try`/`except` block.

## Custom Types

| Rust | Python equivalent |
|------|-------------------|
| Named `struct` | `@dataclass` / `NamedTuple` |
| Tuple struct (`struct Color(i32, i32, i32)`) | `NamedTuple` without field names |
| Unit struct (`struct Marker;`) | Sentinel / marker class |
| `enum` with data | Tagged union — no direct equivalent (`Enum` + `Union`) |
| `type` alias | `TypeAlias` from `typing` |

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

type Kilometers = i32;
```

An enum variant can carry data — `Message::Write(String)` is a single type covering what Python would model as a class hierarchy or `Union`.

## Smart Pointers

| Pointer | Ownership | Thread-safe? | Python parallel |
|---------|-----------|--------------|-----------------|
| `Box<T>` | Single owner, heap-allocated | Yes (`Send` if `T: Send`) | A plain object reference (heap by default) |
| `Rc<T>` | Shared, reference-counted | No | `sys.getrefcount` semantics, manual |
| `Arc<T>` | Shared, atomically counted | Yes | Closest to shared references across threads |
| `Cell<T>` / `RefCell<T>` | Interior mutability | No | Any mutable object — Rust checks borrows at runtime instead |
| `Cow<'a, T>` | Borrowed or owned, clones on write | — | No equivalent; lazily avoids copying |

```rust
let boxed: Box<i32> = Box::new(5);   // heap value, single owner
```

> [!NOTE]
> `Rc`/`Arc` cycles leak (reference counts never reach zero). Break back-edges with `Weak` — see [Ownership](./rust-ownership.md).

## Key Takeaways

1. `Option` and `Result` move null checks and error handling from runtime convention to compile-time obligation.
2. Enums-with-data replace whole Python class hierarchies with a single exhaustive type.
3. Smart pointers make heap strategy explicit where Python hides it behind the garbage collector.

## Next

- [Ownership](./rust-ownership.md) — the rules that make all of the above memory-safe.
