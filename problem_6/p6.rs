// Problem: Sum square difference
// Find the difference between the sum of the squares of 
// the first one hundred natural numbers and the square of the sum.
// https://projecteuler.net/problem=6

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("{}", find_sum_square_difference());

    let elapsed = now.elapsed();
    println!("Elapsed: {:2?}", elapsed);
}

fn find_sum_square_difference() -> i32 {
    // Naive solution to the problem, but the scope for this issue
    // does let me use it
    // For a larger scale this would not work
    let mut i = 1;
    let mut sum = 0;
    let mut sum_of_squares = 0;

    while i <= 100 {
        sum += i;
        sum_of_squares += i*i;

        i += 1;
    }

    let square_of_sums = sum * sum;
    return square_of_sums - sum_of_squares;
}
