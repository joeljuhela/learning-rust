// Largest Prime Factor
// Problem: Find the largest prime factor of 600851475143
// https://projecteuler.net/problem=3

fn main() {
    // This part just calculates the time it takes
    // for this particular solution
    use std::time::Instant;
    let now = Instant::now();

    println!("Largest prime: {}", largest_prime(600851475143));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn largest_prime(n: i64) -> i64 {
    // Finds the largest prime divisor of a number
    // This is a naive solution
    // If none are found (n is 0,1 or a prime number) returns -1
    let mut l_prime = -1;

    let mut i = 2;
    // all unique divisors are smaller than or equal to sqrt(n)
    while i*i < n {
        if n % i == 0 {
            if is_prime(i) {
                l_prime = i;
            }
        }
        i+=1;
    }
    return l_prime;
}



fn is_prime(n: i64) -> bool {
    // Checks if n is a prime number
    // I'm using 6k+-1 optimization to check primality
    // For more information about this, check
    // https://en.wikipedia.org/wiki/Primality_test

    if n > 3 {
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        } else {
            // This method relies on the fact that all primes
            // larger than 3 are of the form 6k+-1
            // where k is any integer larger than 0

            // We don't need to search beyond the point where
            // i is larger than the square root of n, as all 
            // unique divisors of are less than or equal to sqrt(n)
            let mut i = 5;
            while i * i < n {
                if n % i == 0 || n % (i+2) == 0 {
                    return false;
                }
                i += 6;
            }
        }
    }
    return true;
}
