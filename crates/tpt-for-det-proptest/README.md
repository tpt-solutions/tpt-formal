# tpt-for-det-proptest

Deterministic, seedable property-based testing.

Unlike fuzzing harnesses that draw entropy from the OS, this crate drives case
generation from an explicit 64-bit seed via a fixed PRNG. The same seed always
produces the same sequence of cases, so a failing property can be reproduced
exactly — crucial for CI and regression tracking.

## Features

- `DeterministicRng` — a fast xorshift64* generator; `next_u64`, `gen_range`,
  `gen_bool`, `fill_bytes`.
- `Gen<T>` — a strategy producing `T` from a deterministic RNG (impls for
  `u64`/`u32`/`i64`/`bool`).
- `check_prop` — run a property over `N` generated cases, returning
  `Outcome::Passed` or a `CounterExample` (input, seed, case index).
- `assert_prop` — like `check_prop` but panics with the counterexample on
  failure, for use inside `#[test]`s.

## Example

```rust
use tpt_for_det_proptest::{check_prop, DeterministicRng, Gen, Outcome};

// The same seed always yields the same stream.
let mut a = DeterministicRng::new(42);
let mut b = DeterministicRng::new(42);
assert_eq!(a.next_u64(), b.next_u64());

// A fully determined property run (gen, cases, seed) → reproducible result.
let out = check_prop(&(), 100, 1, |x: u64| x.wrapping_add(1) != x.wrapping_sub(1));
assert!(matches!(out, Outcome::Passed { .. }));

// A failing property returns a concrete, replayable counterexample.
let bad = check_prop(&(), 100, 7, |x: u64| x % 2 == 0);
if let Outcome::Failed(ce) = bad {
    assert!(ce.input % 2 != 0);
}
```

## Cargo features

No optional features. `core`-only (`no_std`), no dependencies.

## Integration

The deterministic-engine counterpart to `tpt-for-deterministic-sim`; both share
the same xorshift64* PRNG design so simulation and testing stay reproducible.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.