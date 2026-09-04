// Rust version
// Python:
//   class Counter:
//       def __init__(self):
//           self.value = 0
struct Counter {
    value: i64,
}

impl Counter {
    // Python: def __init__(self):  — but `new` is just a convention, not magic.
    // Called as Counter::new(), like a classmethod that returns an instance.
    fn new() -> Self {
        Counter { value: 0 } // Python: self.value = 0
    }

    fn increment(&mut self) {
        // &mut self = I will modify this
        // Python: def increment(self):  — mutation is implicit; no &mut
        self.value += 1;
    }

    fn get_value(&self) -> i64 {
        // &self = I only read this
        // Python: def get_value(self): return self.value
        self.value
    }
}

fn main() {
    let mut c = Counter::new(); // Python: c = Counter()
    // Must be `mut` to call increment(); Python has no mut binding
    c.increment(); // Python: c.increment()
    println!("{}", c.get_value()); // Python: print(c.get_value())  → 1
}
