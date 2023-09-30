fn find_shortest_word(input: &str) -> Option<&str> {
    let mut shortest_word: Option<&str> = None;
    let mut shortest_length: usize = usize::max_value();

    for word in input.split_whitespace() {
        let word_length = word.len();
        if word_length < shortest_length {
            shortest_word = Some(word);
            shortest_length = word_length;
        }
    }

    shortest_word
}

fn main() {
    let input = "This is a test string with some words.";
    
    match find_shortest_word(input) {
        Some(shortest) => println!("The shortest word is: '{}'", shortest),
        None => println!("No words found in the input."),
    }
}
