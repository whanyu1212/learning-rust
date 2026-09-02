// Rust — ~0.09 seconds for the same 10 million calls
use std::hint::black_box;
use std::time::Instant;

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}

fn main() {
    let start = Instant::now();
    let results: Vec<u64> = (0..10_000_000)
        .map(|n| fibonacci(black_box(n % 30)))  // black_box the input so it can't be optimized away
        .collect();
    black_box(&results);  // black_box the output so the whole computation is "used"
    println!("Elapsed: {:.2?}", start.elapsed());  // ~0.09s
}
