pub fn reverse(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars().rev() {
        output.push(c);
    }
    output
}

// Idiomatic equivalent — collect() builds the String from the iterator:
// pub fn reverse(input: &str) -> String {
//     input.chars().rev().collect()
// }
