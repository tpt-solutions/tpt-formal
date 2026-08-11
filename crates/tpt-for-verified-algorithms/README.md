# tpt-for-verified-algorithms

Verified algorithm implementations guarded by design-by-contract checks.

Each routine is wrapped in `tpt-for-contract` precondition / postcondition /
loop-invariant checks, so calling it with inputs that violate its contract
panics instead of producing a silently-wrong result. Checks are written against
the workspace MSRV (1.75), avoiding newer `std` helpers.

## Features

- `gcd(a, b)` — Euclid's GCD; panics if both inputs are zero.
- `clamp(x, lo, hi)` — saturating clamp into `[lo, hi]`.
- `binary_search(slice, target)` — requires a sorted slice, returns the index.
- `insertion_sort(input)` — returns a sorted permutation of the input.
- All functions document their contracts in their doc comments and enforce them
  with the `requires!` / `ensures!` / `invariant!` macros.

## Example

```rust
use tpt_for_verified_algorithms::{gcd, clamp, binary_search, insertion_sort};

assert_eq!(gcd(12, 8), 4);
assert_eq!(gcd(17, 0), 17);

assert_eq!(clamp(5, 0, 10), 5);
assert_eq!(clamp(-3, 0, 10), 0);
assert_eq!(clamp(99, 0, 10), 10);

assert_eq!(binary_search(&[1, 3, 5, 7, 9], 5), Some(2));

let sorted = insertion_sort(&[3, 1, 2, 5, 4]);
assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
```

## Cargo features

No optional features. Depends on `tpt-for-contract`.

## Integration

A showcase consumer of `tpt-for-contract`: the same contract surface is used by
`tpt-for-verified-ode` for numerically-sensitive integrators.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.