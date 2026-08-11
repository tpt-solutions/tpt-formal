# tpt-for-assert-const

Compile-time assertions (const_assert / const_assert_eq / type-level checks).

Catch invariant violations *while compiling* instead of at runtime. Everything is
`no_std`.

## Features

- `const_assert!(cond)` — fail the build if `cond` is false at compile time.
- `const_assert_eq!(a, b)` / `const_assert_ne!(a, b)` — compile-time equality
  and inequality checks.
- `Const<bool>`, `IsTrue` and `Same` — type-level machinery that lets a trait
  bound *require* a boolean to be `true` or two types to be equal.

## Example

```rust
tpt_for_assert_const::const_assert!(2 + 2 == 4);
tpt_for_assert_const::const_assert!(core::mem::size_of::<u32>() == 4);
tpt_for_assert_const::const_assert_eq!(1u8 + 1, 2u8);

// A trait bound that forces B to be true at compile time:
fn requires_true<B: tpt_for_assert_const::IsTrue>() {}
requires_true::<tpt_for_assert_const::Const<true>>();
```

## Cargo features

The crate is `#![no_std]` and uses only `core`. The `std` and `alloc` features
are declared for downstream convenience; the crate has no `std`-only code paths.

## Integration

Used wherever a compile-time invariant must be guaranteed before codegen — for
example to guard generic implementations in the rest of the `tpt-formal`
workspace.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.