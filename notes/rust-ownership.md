# Rust Ownership

The core of Rust's memory safety — and the reason circular references are nearly impossible to construct by accident.

## The core rule

Every value in Rust has **exactly one owner** at any moment. When the owner goes out of scope, the value is dropped (memory freed) automatically.

```rust
let a = String::from("hello"); // `a` owns the String
let b = a;                     // ownership MOVES to `b`; `a` is now invalid
// println!("{a}");            // compile error — use after move
```

## Why this kills circular references

In GC languages (Java, Python, JS), objects hold *pointers* to each other. If A points to B and B points back to A, neither is ever "unreachable" — the garbage collector can't collect the cycle. Classic memory leak.

Rust's ownership model makes this **impossible by default**:

1. **You can't have two owners.** A cycle needs A to own B *and* B to own A. But only one owner exists. If A owns B, then B owning A would require B to own the thing that owns it — a self-referential structure.

2. **Borrows can't create cycles.** You could try:
   ```rust
   struct Node {
       value: i32,
       parent: Option<&'a Node>, // borrow, not ownership
   }
   ```
   The borrow checker rejects this. A borrow (`&`) must live *inside* the owner's lifetime. You can't have B borrow A while A owns B — the borrow of A would outlive A. It's a lifetime error.

So the **compile-time guarantee** is: with plain ownership + borrowing, you physically cannot construct a cycle. The borrow checker won't let you write code that does.

## Ownership by composition — the safe default

The idiomatic way to build a tree (parent owns children) uses **owned inline** values:

```rust
struct Node {
    value: String,
    children: Vec<Node>,  // Children are OWNED — no cycles possible
}

impl Node {
    fn new(value: &str) -> Self {
        Node { value: value.to_string(), children: Vec::new() }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);  // Ownership transfers here
    }
}
```

Here each node is a direct value inside its parent, not a pointer. No node can own its ancestor, because there's no pointer to loop back. When the root drops, all children drop recursively — deterministic, zero overhead, no GC. This design *can't even express* a cycle.

## Where cycles *can* happen — `Rc`

To build a graph or parent/child tree where children know their parent, you need **shared ownership** — something both A and B can refer to. That's `Rc<T>` (reference counting):

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Option<Rc<Node>>>, // Rc = shared ownership
}
```

Now A and B *can* point to each other. But `Rc` uses **runtime** reference counting — when the last `Rc` drops, memory frees. A cycle means the counts never reach zero:

```
A.parent -> B
B.children -> A
```

Neither count ever hits 0 → **memory leak**. This is the exact problem Rust's ownership normally prevents, now reintroduced through the back door.

## The fix: `Weak`

The Rust-idiomatic solution is `Weak<T>` — a **non-owning** reference. It does *not* keep the value alive. Combine it with `Rc`:

```rust
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: RefCell<Option<Weak<Node>>>, // Weak breaks the cycle
    children: RefCell<Vec<Rc<Node>>>,
}
```

Now:
- Parent → child: `Rc` (strong, owns, keeps alive)
- Child → parent: `Weak` (weak, doesn't own, doesn't keep alive)

When the last strong `Rc` drops, the `Node` frees even if weak refs still exist. To use a `Weak`, you `upgrade()` it → returns `Option<Rc>` (None if the value already died).

## Summary table

| Mechanism | Can it make a cycle? | Cycle consequence | Solution |
|-----------|---------------------|-------------------|----------|
| Ownership + `&` borrow | **No** — borrow checker rejects it | Impossible | None needed |
| `Rc` + `RefCell` | Yes | Runtime memory leak (refcount never hits 0) | Use `Weak` for back-edges |
| `Arc` (thread-safe `Rc`) | Yes | Same leak | Use `Weak` |

## Key takeaway

Rust's ownership model is designed so you *can't* create circular references by accident. When you genuinely need a cyclic structure (graphs, parent pointers), you must opt into `Rc`/`Arc` — and then it's your job to break the cycle with `Weak` on the "back" edge. The compile-time safety gives way to a runtime discipline you have to enforce yourself.
