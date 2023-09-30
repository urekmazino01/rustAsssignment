fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; // 0 and 1 are not prime numbers
    }

    if n == 2 {
        return true; // 2 is a prime number
    }

    if n % 2 == 0 {
        return false; // Even numbers (except 2) are not prime
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false; // Found a divisor, not a prime number
        }
        i += 2; // Check odd numbers only
    }

    true // No divisors found, it's a prime number
}

fn main() {
    let num = 17; // Change this to the number you want to check for primality

    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}
