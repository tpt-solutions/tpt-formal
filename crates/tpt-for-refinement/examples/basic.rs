//! Example: a refined value is a base value guaranteed to satisfy a predicate.
use tpt_for_refinement::Predicate;
use tpt_for_refinement::Refined;

struct Positive;
impl Predicate<i64> for Positive {
    fn check(value: &i64) -> bool {
        *value > 0
    }
}

fn main() {
    let r = Refined::<i64, Positive>::new(7).expect("must be positive");
    println!("refined value = {}", r.get());
}
