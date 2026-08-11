//! Example: run a small deterministic simulation over a custom world.
use tpt_for_deterministic_sim::{SimRng, Simulation, World};

struct Counter;
impl World for Counter {
    type State = i64;
    type Action = i64;
    type Event = i64;
    fn step(&self, s: &i64, a: &i64, _rng: &mut SimRng) -> (i64, i64) {
        (s + a, s + a)
    }
}

fn main() {
    let mut sim = Simulation::new(Counter, 0, 1);
    sim.run([1i64, 2, 3]);
    println!("final state = {}", sim.state());
}
