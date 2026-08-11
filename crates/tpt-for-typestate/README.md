# tpt-for-typestate

Type-level safety encoding toolkit: phantom/ghost/newtype/bounded/safe-cast.

`tpt-for-typestate` bundles the standard techniques for encoding program
invariants *in the Rust type system* so invalid states become unrepresentable.
It is `no_std`, core-only, and zero-dependency.

## Features

- `phantom` — variance-controlled markers `Invariant` / `Covariant` /
  `Contravariant` (more precise than `PhantomData<T>`), plus a `Sealed` trait
  for the sealed-trait pattern.
- `ghost` — `State`/`Token` typestate machinery and `Ghost<P, T>` ghost values
  that carry a logical predicate `P` as an erased (zero-cost) phantom.
- `newtype` — a `Newtype` conversion trait and the `define_newtype!` macro for
  transparent newtypes with derived `Debug`/`Clone`/`Copy`/`PartialEq`/`Ord`/
  `Hash` and `From`/`Into` glue.
- `bounded` — `Bounded<T>` for runtime range checks and `Checked<B>` for
  type-level named bounds via the `Bound` trait.
- `safe_cast` — infallible widening casts (`SafeCast`) and fallible narrowing /
  cross-kind casts (`TrySafeCast`) that error instead of truncating.

## Example

```rust
use tpt_for_typestate::bounded::{Bounded, Checked, Bound};
use tpt_for_typestate::safe_cast::safe_cast;

// Transparent newtype with From/Into glue.
tpt_for_typestate::define_newtype!(Meters, u64);
let m = Meters::from_inner(3);
assert_eq!(u64::from(m), 3);

// Runtime range-checked value that rejects out-of-range input.
let b = Bounded::new(5u8, 0, 10).expect("in range");
assert_eq!(b.saturate(20).get(), 10);

// Type-level named bound.
struct Pct;
impl Bound for Pct {
    type Value = u8;
    const MIN: u8 = 0;
    const MAX: u8 = 100;
}
assert!(Checked::<Pct>::new(50).is_some());
assert!(Checked::<Pct>::new(101).is_none());

// Infallible widening cast (never loses information).
let wide: u64 = 200u8.safe_cast();
```

## Cargo features

The crate is `#![no_std]` and uses only `core`. The `std` and `alloc` features
are declared for downstream convenience; the crate currently has no `std`-only
code paths.

## Integration

This is a foundational crate. `tpt-for-witness` and `tpt-for-contract` build on
its phantom/variance machinery, and `tpt-for-refinement` composes with its
type-level tools.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.