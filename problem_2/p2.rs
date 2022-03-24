// Even Fibonacci Numbers
// Problem: Find the sum of the even-valued terms in the 
// Fibonacci sequence in whose values do not exceed four million
// https://projecteuler.net/problem=2

fn main() {
    // This calculates the time it takes to run
    // the solution.
    use std::time::Instant;
    let now = Instant::now();

    fib_even_sum();

    let elapsed = now.elapsed();
    println!("Elapsed {:.2?}", elapsed);
}


fn fib_even_sum() {
    // Create a vector for storing the numbers for calculating
    // The next number in the sequence
    let mut vector = vec![0,1];
    let mut final_sum = 0;

    while vector[1] < 4000000 {
        let next_member = vector[0] + vector[1];
        vector.remove(0);
        vector.push(next_member);

        if next_member % 2 == 0 {
            final_sum += next_member;
        }
    }

    println!("SUM: {}", final_sum);
}
