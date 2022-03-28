// Problem: Smallest multiple
// What is the smallest positive number that
// is evenly divisible by all of the numbers from 1 to 20?

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("{}", find_smallest_multiple_naive());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn find_smallest_multiple_naive() -> i32 {
    // This is a naive brute force option for 
    // solving the problem

    let mut num = 21;
    
    loop {
        let mut i = 1;
        let mut flag = true;
        while i <= 20 {
            if !(num % i == 0) {
                flag = false;
                break;
            }
            i += 1;
        }
        if flag {
            return num;
        }
        num += 1;
    }
}