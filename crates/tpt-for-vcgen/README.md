# tpt-for-vcgen

Verification-condition generation (VCGen).

Lowers design-by-contract annotations to verification conditions using the
weakest-precondition (WP) calculus, then emits them as a `tpt-for-smt-lite`
`Problem` so they can be discharged by an SMT solver. This is the bridge between the
`requires!`/`ensures!` contract style and `tpt-for-smt-lite`'s solver-agnostic term
language. (Only depends on `tpt-for-smt-lite`; it does not use `tpt-for-contract`.)

For a program `pre { body } post`, the generated VC is `pre ∧ wp(body, post)`,
and the program is verified exactly when its *negation* is unsatisfiable.

## Features

- `Expr` / `BExpr` — integer expressions and boolean conditions with `subst`
  and free-variable collection.
- `Stmt` — `Assign`, `Assume`, and `Skip` over annotated programs.
- `Spec` — a `pre`/`post` contract.
- `wp(body, post)` — weakest-precondition computation.
- `generate_vc(spec, body)` — build the (negated) VC as an `smt_lite::Problem`.
- `verify(spec, body)` — discharge with the built-in ground evaluator, returning
  `Verified` / `Falsified` / `Inconclusive`.

## Example

```rust
use tpt_for_vcgen::{Expr, BExpr, Stmt, Spec, verify, VcResult};

// x := 1; x := x + 1;   post: x == 2
let spec = Spec {
    pre: BExpr::bool(true),
    post: BExpr::eq(Expr::var("x"), Expr::const_(2)),
};
let body = vec![
    Stmt::assign("x", Expr::const_(1)),
    Stmt::assign("x", Expr::add(Expr::var("x"), Expr::const_(1))),
];
assert!(matches!(verify(&spec, &body), VcResult::Verified));
```

## Cargo features

No optional features. Depends on `tpt-for-smt-lite`.

## Integration

Produces obligations for `tpt-for-smt-lite` and is itself wired into
`tpt-for-smt-lite`'s SAT-backed decision tier; together they form the automated
verification pipeline in the workspace.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.