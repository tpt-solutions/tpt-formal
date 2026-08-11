//! Example: verified algorithm building blocks.
use tpt_for_verified_algorithms::{clamp, gcd, insertion_sort};

fn main() {
    println!("gcd(12, 18) = {}", gcd(12, 18));
    println!("clamp(5, 0, 3) = {}", clamp(5, 0, 3));
    println!(
        "insertion_sort([3, 1, 2]) = {:?}",
        insertion_sort(&[3, 1, 2])
    );
}
