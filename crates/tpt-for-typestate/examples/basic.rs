//! Example: range-checked numeric values via `bounded::Bounded`.
use tpt_for_typestate::bounded::Bounded;

fn main() {
    let b = Bounded::new(5u32, 0u32, 10u32).expect("in range");
    println!("value = {} (clamped into [0, 10])", b.get());
}
