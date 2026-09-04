# Data Types: Collections, References & Functions

> **TL;DR** — `Vec` ≈ Python `list`, `HashMap` ≈ `dict`, `HashSet` ≈ `set`. References (`&T`, `&mut T`) are explicit, compiler-checked borrows — Python references are implicit and unchecked.

## Collections

| Rust | Python equivalent | Notes |
|------|-------------------|-------|
| `Vec<T>` | `list` | Growable, heap-allocated, single element type |
| `HashMap<K, V>` | `dict` | Hash-based; both key and value types uniform |
| `HashSet<T>` | `set` | Unique values, hash-based |
| `VecDeque<T>` | `collections.deque` | Double-ended queue |
| `BTreeMap<K, V>` / `BTreeSet<T>` | `sortedcontainers` / `bisect` + `dict` | Sorted; no stdlib equivalent in Python |
| `BinaryHeap<T>` | `heapq` | Priority queue (max-heap in Rust) |

### Vector (`Vec<T>`)

```rust
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];
```

```python
v = [1, 2, 3]
```

Unlike Python lists, every element must share one type — mixed `[1, "a"]` does not compile.

### HashMap (`HashMap<K, V>`)

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
```

```python
scores = {"Blue": 10}
```

### HashSet (`HashSet<T>`)

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(1);
```

```python
s = {1}
```

## Reference Types

| Rust | Meaning | Python parallel |
|------|---------|-----------------|
| `&T` | Shared (immutable) borrow | A read-only view — no direct equivalent; closest is passing an object you promise not to mutate |
| `&mut T` | Exclusive (mutable) borrow | Passing a mutable object, but Rust guarantees **only one** mutable borrow exists |
| `*const T` / `*mut T` | Raw pointers, `unsafe` to dereference | `ctypes` pointers — escape hatch, unchecked |

```rust
let x = 5;
let y = &x;        // shared borrow
let mut z = 10;
let w = &mut z;    // exclusive borrow
```

The rule that trips up Python programmers: you can have **many** `&T` *or* **one** `&mut T` — never both at once. The compiler rejects violations. See [Ownership](./rust-ownership.md#when-to-own-vs-borrow) for when to own vs borrow, and [shared vs exclusive](./rust-ownership.md#shared-vs-exclusive-borrows) for which borrow to pick.

> [!WARNING]
> Raw pointers (`*const T`, `*mut T`) skip borrow checking entirely and require an `unsafe` block to dereference. Reach for references first.

## Functions, Closures, Unit & Never

Functions have their own pointer type; closures capture their environment like Python lambdas but with explicit ownership semantics:

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
let f: fn(i32) -> i32 = add_one;

let add = |x, y| x + y;   // cf. Python: add = lambda x, y: x + y
```

| Type | Meaning | Python parallel |
|------|---------|-----------------|
| `()` (unit) | Absence of a value; every function without `-> T` returns it | `None` as a return value |
| `!` (never) | Function never returns (e.g. `loop {}`, `panic!`) | `NoReturn` from `typing` |

```rust
fn do_something() {
    println!("doing something");  // returns () implicitly
}
```

## Key Takeaways

1. Collections map 1:1 onto Python's `list`/`dict`/`set` — the new constraint is uniform element types.
2. References make Python's implicit borrowing explicit and compiler-checked.
3. `&T` vs `&mut T` exclusivity is the mechanism behind data-race safety in [Concurrency without a GIL](./concurrency-gil.md).

## Next

- [Option, Result, Custom Types & Smart Pointers](./rust-custom-types.md) — null-safety, error handling, and user-defined types.
