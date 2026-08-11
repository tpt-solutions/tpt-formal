# tpt-for-symbolic-exec

Whole-program symbolic execution (clean-room).

Interprets a small imperative language over *symbolic* values instead of
concrete ones, accumulating a path condition (PC) for every explored path and
pruning infeasible paths with an SMT backend (`tpt-for-smt-lite`). Two defects
are detected automatically:

- **Division by zero** — a `Div` whose denominator can be `0` on a feasible
  path.
- **Broken assertions** — an `Assert` whose negation is feasible.

## Features

- `SExpr` / `SCond` / `SStmt` — a symbolic IR with `Assign`, `If`, `Assert`,
  and `Assume`, plus symbolic inputs via `SExpr::sym`.
- `run(program)` — explore all feasible paths, returning a `SymReport` of
  `violations` (each carrying its witnessing path condition) and
  `paths_explored`.
- Infeasible branches are pruned using `tpt-for-smt-lite`'s satisfiability
  check; `Unknown` results are treated conservatively as feasible.

## Example

```rust
use tpt_for_symbolic_exec::{SExpr, SCond, SStmt, run, ViolationKind};

// z = 10 / x;   with x a free symbolic input → x == 0 is feasible.
let prog = vec![SStmt::assign(
    "z",
    SExpr::div(SExpr::const_(10), SExpr::sym("x")),
)];
let report = run(&prog);
assert_eq!(report.violations.len(), 1);
assert!(matches!(report.violations[0].kind, ViolationKind::DivByZero));

// A guarded, ground-true assertion produces no violations:
let ok = vec![
    SStmt::assume(SCond::eq(SExpr::const_(2), SExpr::const_(2))),
    SStmt::assert(SCond::eq(
        SExpr::add(SExpr::const_(2), SExpr::const_(3)),
        SExpr::const_(5),
    )),
];
assert!(run(&ok).violations.is_empty());
```

## Cargo features

No optional features. Depends on `tpt-for-smt-lite`.

## Integration

A dynamic counterpart to `tpt-for-abstract-interp`, sharing the
`tpt-for-smt-lite` backend for feasibility; both feed the workspace's
verification tooling.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.