# tpt-for-abstract-interp

Generic abstract-interpretation framework.

Provides the three ingredients of abstract interpretation:

1. An `AbstractDomain` trait — the elements of an abstract lattice with
   `join` / `meet` / `widen`.
2. An `Interval` domain over `i64` — the canonical first domain.
3. A fixpoint `analyze` engine over a control-flow graph (`Cfg`) of
   assignment/assume basic blocks, using a worklist with widening to guarantee
   termination on loops.

## Features

- `AbstractDomain` — `top` / `bottom` / `join` / `meet` / `widen` contract;
  implemented for `Interval` and `Vec<Interval>` (abstract state vectors).
- `Interval` — saturating, over-approximating interval arithmetic (`[lo, hi]`).
- `Expr` / `Stmt` / `CmpOp` / `Cfg` / `Block` — a small imperative IR with
  `assign` and `assume` (narrowing) statements.
- `analyze(cfg, entry, init)` — worklist fixpoint with widening; returns the
  entry abstract state for every node (or `None` on bad input).

## Example

```rust
use tpt_for_abstract_interp::{AbstractDomain, Cfg, Stmt, Expr, CmpOp, Interval, analyze};

// x = 0; while (x < 10) { x = x + 1; }  →  x ∈ [10, +∞) at exit
let mut cfg = Cfg::new(1);
cfg.block(&[Stmt::assign(0, Expr::const_(0))], &[1]);
cfg.block(&[], &[2, 3]);                                   // loop head
cfg.block(
    &[Stmt::assume(0, CmpOp::Lt, 10),
       Stmt::assign(0, Expr::add(Expr::var(0), Expr::const_(1)))],
    &[1],
);
cfg.block(&[Stmt::assume(0, CmpOp::Ge, 10)], &[4]);
cfg.block(&[], &[]);                                       // exit

let states = analyze(&cfg, 0, vec![Interval::bottom()]).unwrap();
let exit = states[4][0];
assert_eq!(exit.lo(), 10);
assert_eq!(exit.hi(), i64::MAX);
```

## Cargo features

No optional features. Pure `std` (no external dependencies).

## Integration

One of the static-analysis engines of the workspace; complements
`tpt-for-symbolic-exec` (which reasons about symbolic paths) and feeds the same
`tpt-for-proof-ast` obligation model.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.