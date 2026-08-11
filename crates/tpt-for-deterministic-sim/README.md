# tpt-for-deterministic-sim

Deterministic, seedable simulation harness.

A `Simulation` advances a `World` one step at a time from a seeded PRNG,
recording each step so a run can be replayed and inspected. Because the RNG is
deterministic, identical `(world, actions, seed)` inputs always produce
identical logs — essential for reproducible fault analysis and debugging.

## Features

- `SimRng` — the same fast xorshift64* generator used by `tpt-for-det-proptest`.
- `World` — implement `State` / `Action` / `Event` and a `step` transition.
- `Simulation` — `new(world, initial, seed)`, `apply(action)`, `run(actions)`;
  records every `StepRecord` and exposes `state()` / `steps()`.
- Fully determined by `(world, actions, seed)` — replay by re-running.

## Example

```rust
use tpt_for_deterministic_sim::{Simulation, World, SimRng};

struct Counter;
impl World for Counter {
    type State = i64;
    type Action = i64;
    type Event = i64;
    fn step(&self, s: &i64, a: &i64, _rng: &mut SimRng) -> (i64, i64) {
        let next = s + a;
        (next, next)
    }
}

let mut sim = Simulation::new(Counter, 0, 123);
sim.run([1i64, 2, 3, 4]);
assert_eq!(*sim.state(), 10);
assert_eq!(sim.steps().len(), 4);

// Identical seed + inputs reproduce the run byte-for-byte.
let mut sim2 = Simulation::new(Counter, 0, 123);
sim2.run([1i64, 2, 3, 4]);
assert_eq!(sim.steps(), sim2.steps());
```

## Cargo features

No optional features. `core`-only (`no_std`), no dependencies.

## Integration

Shares its RNG design with `tpt-for-det-proptest` and is the runtime harness
counterpart to the analysis crates (`tpt-for-model-check`,
`tpt-for-redundancy`) that reason about such traces.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.