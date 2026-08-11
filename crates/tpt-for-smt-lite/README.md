# tpt-for-smt-lite

Lightweight SMT bridge: typed term/problem builder, SMT-LIB2 serializer, minimal evaluator.

The *bridge surface* for talking to an SMT solver: build a typed problem in
Rust, serialize it to SMT-LIB 2, and (with an external solver wired in) check
satisfiability or extract a model. It ships with a minimal built-in evaluator for
**ground** formulas, so it is usable and testable without an external solver
binary.

## Features

- `Term` / `Sort` / `Value` — a typed expression language (Bool, Int, BitVec)
  with a fluent builder API and `core::ops` overloads.
- `Problem` — declare constants and assert terms; `to_smtlib2()` serializes a
  complete SMT-LIB 2 script.
- `check_sat()` — a built-in ground evaluator returning `Sat` / `Unsat` /
  `Unknown` (free variables → `Unknown`).
- `get_model()` — extract a (possibly partial) model.

## Example

```rust
use tpt_for_smt_lite::{Problem, Sort, Term, SatResult};

let mut p = Problem::new();
p.declare_const("x", Sort::Int);
p.assert(Term::var("x").le(Term::int(10)));

// The built-in evaluator cannot decide a formula with free variables…
assert_eq!(p.check_sat(), SatResult::Unknown);

// …but it decides fully ground formulas:
let mut q = Problem::new();
q.assert(Term::int(1).equals(Term::int(2)));
assert_eq!(q.check_sat(), SatResult::Unsat);

// Serialize for an external solver:
println!("{}", p.to_smtlib2());
```

## Cargo features

No optional features. Per ADR 0007 the acceptable external backends `rsmt2`
(MIT/Apache-2.0) and `z3` (MIT) are documented but not yet depended upon; the
`backend-rsmt2` wrap is deferred until a solver binary is guaranteed in CI.

## Integration

Provides the term/problem abstraction consumed by `tpt-for-vcgen` and
`tpt-for-symbolic-exec` to discharge verification conditions and path
feasibility.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.