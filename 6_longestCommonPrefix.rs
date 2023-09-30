fn longest_common_prefix(strings: Vec<String>) -> String {
    if strings.is_empty() {
        return String::new(); // Return an empty string if the input vector is empty.
    }

    let mut common_prefix = strings[0].clone(); // Initialize with the first string.

    for s in strings.iter().skip(1) {
        // Compare each string in the vector with the common prefix.
        // Update the common prefix to match the characters shared with the current string.
        common_prefix = common_prefix
            .chars()
            .zip(s.chars())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect();
    }

    common_prefix
}

fn main() {
    let input_strings = vec![
        "flower".to_string(),
        "flour".to_string(),
        "flow".to_string(),
    ];

    let result = longest_common_prefix(input_strings);

    if result.is_empty() {
        println!("There is no common prefix.");
    } else {
        println!("Longest common prefix: {}", result);
    }
}
