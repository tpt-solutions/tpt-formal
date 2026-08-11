# tpt-for-witness

Witness types: values carrying compile-time proof that a predicate holds.

A *witness* bundles a value with a zero-sized proof that some `Predicate` holds
of it. Construction is gated by the predicate's runtime (or trusted) check, so a
function receiving a `Witness<P, T>` can rely on `P` being true without
re-checking. Built on `tpt-for-typestate` for variance-correct phantom storage.

## Features

- `Witness<P, T>` — a value `T` carrying a compile-time proof that
  `P: Predicate<T>` holds. Construct via `try_new` (verified) or
  `new_unchecked` (trusted, `unsafe`).
- `Predicate<T>` — describe how to verify a property; built-in predicates
  `Positive`, `NonNegative`, `NonZero` (for numeric `Zero` types) and
  `NonEmpty` (for slices).
- `And<P, Q>` — logical conjunction of two predicates, with `AndError` to
  surface which side(s) failed.
- `map` / `into_inner` / `AsRef` — work with the inner value while preserving the
  proof.

## Example

```rust
use tpt_for_witness::{Witness, Positive, NonEmpty, And};

// A value known to be strictly positive.
let w = Witness::<Positive, i32>::try_new(5).unwrap();
assert_eq!(*w.as_ref(), 5);
assert!(Witness::<Positive, i32>::try_new(-1).is_err());

// A non-empty slice, verified at construction.
let data = [1u8, 2, 3];
let w = Witness::<NonEmpty, _>::try_new(&data[..]).unwrap();
assert_eq!(w.as_ref().len(), 3);

// Compose predicates: non-negative AND non-zero.
type NonNegNonZero = And<tpt_for_witness::NonNegative, tpt_for_witness::NonZero>;
assert!(Witness::<NonNegNonZero, i32>::try_new(5).is_ok());
assert!(Witness::<NonNegNonZero, i32>::try_new(0).is_err());
```

## Cargo features

The crate is `#![no_std]` and uses only `core`. The `std` and `alloc` features
are declared for downstream convenience.

## Integration

Builds on `tpt-for-typestate`. Pairs naturally with `tpt-for-refinement` for
stronger, compile-time-checked value constraints.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.