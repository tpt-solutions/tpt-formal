//! Example: drive a deterministic property check from a fixed seed.
use tpt_for_det_proptest::{check_prop, DeterministicRng, Gen};

struct Unit;
impl Gen<u64> for Unit {
    fn generate(&self, rng: &mut DeterministicRng) -> u64 {
        rng.next_u64()
    }
}

fn main() {
    let out = check_prop(&Unit, 100, 7, |x: u64| {
        x.wrapping_add(1) != x.wrapping_sub(1)
    });
    println!("{:?}", out);
}
