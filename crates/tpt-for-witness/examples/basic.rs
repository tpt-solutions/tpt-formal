//! Example: a witness bundles a value with proof that it satisfies a predicate.
use tpt_for_witness::{Positive, Witness};

fn main() {
    let w = Witness::<Positive, i64>::try_new(42).expect("must be positive");
    println!("witness value = {}", w.into_inner());
}
