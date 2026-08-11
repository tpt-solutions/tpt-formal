//! Example: declaring and checking a struct invariant.
use tpt_for_contract::Invariant;

struct NonEmptyVec<T> {
    data: Vec<T>,
}

impl<T> Invariant for NonEmptyVec<T> {
    fn check(&self) -> bool {
        !self.data.is_empty()
    }
}

fn main() {
    let v = NonEmptyVec {
        data: vec![1, 2, 3],
    };
    assert!(v.check());
    println!("invariant holds: {}", v.check());
}
