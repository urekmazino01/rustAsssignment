use std::io;

fn is_palindrome(s: &str) -> bool {
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let s = s.to_lowercase();

    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        let left_char = s.chars().nth(left).unwrap();
        let right_char = s.chars().nth(right).unwrap();

        if left_char != right_char {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}


fn main() {
    println!("Enter a string:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();

    if is_palindrome(input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
