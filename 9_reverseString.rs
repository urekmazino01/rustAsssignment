fn reverse_string(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars().rev() {
        result.push(c);
    }

    result
}

fn main() {
    let original = "Hello, world!";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
