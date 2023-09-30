use std::io;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("Enter a number to check for primality:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num = input.trim().parse::<u64>().expect("Invalid input. Please enter a positive integer.");

    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}
