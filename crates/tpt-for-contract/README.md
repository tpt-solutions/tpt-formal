# tpt-for-contract

Design-by-contract: preconditions, postconditions, invariants, loop invariants.

Consolidates the four facets of one contract concern into a single coherent
surface. All checks are `no_std` and panic with the offending condition's source
text, file, and line. A debug-only variant is provided for checks that should be
skipped in release builds.

## Features

- `requires!(cond)` / `requires!(cond, msg)` — precondition, checked at function
  entry.
- `ensures!(cond)` / `ensures!(cond, msg)` — postcondition, checked at return.
- `invariant!(cond)` — a value's invariant, checked in place.
- `loop_invariant!(cond)` — verified on every loop iteration.
- `debug_requires!` / `debug_ensures!` / `debug_invariant!` /
  `debug_loop_invariant!` — compile to nothing under
  `cfg(not(debug_assertions))`.
- `ContractError` / `report` — the structured failure type and reporter hook.

## Example

```rust
use tpt_for_contract::requires;

fn divide(a: i32, b: i32) -> i32 {
    requires!(b != 0, "division by zero");
    a / b
}

// Postconditions and loop invariants follow the same shape:
use tpt_for_contract::{ensures, loop_invariant};

fn abs(x: i32) -> i32 {
    let mut r = if x < 0 { -x } else { x };
    loop_invariant!(r >= 0);
    ensures!(r >= 0);
    r
}
```

## Cargo features

The crate is `#![no_std]` and uses only `core`. The `std` and `alloc` features
are declared for downstream convenience.

## Integration

This is the contract surface consumed by `tpt-for-refinement`,
`tpt-for-verified-algorithms`, and `tpt-for-verified-ode`, which guard their
logic with these macros.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.