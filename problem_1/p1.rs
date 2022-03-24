// Multiples of 3 or 5
// Problem: Find the sum of all the multiples of 3 or 5 below 1000
// https://projecteuler.net/problem=1

fn main() {
    let mut final_sum = 0;

    let mut i3 = 3;

    // I am using two loops, one for multiples of 3 and a second
    // one for multiples of 5. 
    // This reduces the number of needed iterations from 1000
    // to ~500
    while i3 < 1000 {
        // If the number is both a multiple of 5 and 3, we add it to the sum
        // in the loop for multiples of 5
        if i3 % 5 == 0 {
        } else {
            final_sum += i3
        }
        i3 += 3;
    }

    let mut i5 = 5;
    while i5 < 1000 {
        final_sum += i5;
        i5 += 5;
    }

    println!("SUM: {}", final_sum)
}