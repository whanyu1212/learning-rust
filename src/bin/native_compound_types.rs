fn main() {
    // Type annotation explicit: (i32, f64, u8)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Without type annotation - Rust infers the types automatically!
    // Rust will infer: (i32, f64, i32) based on the literal values
    // Integer literals default to i32, floating-point literals default to f64
    let tup2 = (500, 6.4, 1);
    println!("The third value is: {}", tup2.2);

    // Type annotation explicit: [i32; 5]
    let lst: [i32; 5] = [1, 2, 3, 4, 5];
    let first = lst[0];
    println!("The first element of the array is: {}", first);

    // Without type annotation - Rust infers [i32; 5] automatically
    // It knows there are 5 elements and they're all i32 (default integer type)
    let lst2 = [1, 2, 3, 4, 5];
    println!("The second element of lst2 is: {}", lst2[1]);
}