// Largest Palindrome Product
// Problem: Find the largest palindrome made from 
// the product of two 3-digit numbers
// https://projecteuler.net/problem=4

fn main() {
    // This part just calculates how much
    // time this solution takes
    use std::time::Instant;
    let now = Instant::now();

    println!("{}", find_largest_palindrome());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn find_largest_palindrome() -> i32 {
    // Finds the largest palindrome which is the product
    // of two 3-digit numbers
    let mut first = 999;
    let mut second = 999;

    let mut largest_mult = 0;

    while second >= 100 {
        let mult = first * second;
        if pal_check(mult) && mult > largest_mult {
            largest_mult = mult;
        }
        if first == 100 {
            first = second;
            second = second - 1;
        }

        first = first - 1;
    }
    return largest_mult;
}

fn pal_check(n: i32) -> bool {
    // Checks if a number is a palindrome
    if n < 10 && n > 0 {
        // All numbers under 10 are palindromes
        return true;
    }
    // If the number is 10 or higher,
    // The number is reversed and then checked if there 
    // is a match
    let mut r = 0;
    let mut c = n;

    loop {
        r = r * 10;
        let remainder = c % 10;
        r = r + remainder;
        c = c / 10;

        if c == 0 {break;}
    } 

    return r == n;
}