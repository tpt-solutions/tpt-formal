# tpt-for-sat

From-scratch, pure-Rust CDCL SAT solver (no FFI): CNF, watched literals, 1UIP clause learning, restarts.

No external solver binary, no FFI. Implements a modern conflict-driven
clause-learning (CDCL) loop: watched literals for cheap unit propagation, 1UIP
conflict analysis producing a learned clause on every conflict, VSIDS activity
ordering for decisions, and geometric restart scheduling. The public surface is
tiny: build a `Cnf`, wrap it in a `Solver`, call `solve`.

## Features

- `Cnf` / `Lit` / `Clause` — DIMACS-style representation with dense literal
  indexing and a `from_lits` builder.
- `Solver` — full CDCL engine; `solve()` returns `Sat` / `Unsat`,
  `model()` / `value(var)` expose the satisfying assignment.
- Watched literals, 1UIP learning, VSIDS, and restarts — all implemented
  in-tree with zero dependencies.

## Example

```rust
use tpt_for_sat::{Cnf, Solver, SatResult};

// (x) ∧ (¬x ∨ y) ∧ (¬y ∨ z) ∧ (¬z)  → UNSAT
let cnf = Cnf::from_lits(3, &[
    &[1],
    &[-1, 2],
    &[-2, 3],
    &[-3],
]);
let mut solver = Solver::new(cnf);
assert_eq!(solver.solve(), SatResult::Unsat);

// A satisfiable instance exposes a model:
// (x ∨ y) ∧ (¬x ∨ y) ∧ (x ∨ ¬y)  → SAT (x = y = true)
let cnf2 = Cnf::from_lits(2, &[&[1, 2], &[-1, 2], &[1, -2]]);
let mut s2 = Solver::new(cnf2);
assert_eq!(s2.solve(), SatResult::Sat);
assert!(s2.value(0).unwrap() && s2.value(1).unwrap());
```

## Cargo features

No optional features. Pure `std` (no external dependencies), single-file CDCL
implementation.

## Integration

The Boolean core that higher-level solvers build on; `tpt-for-smt-lite` and
`tpt-for-vcgen` provide the richer theories and obligations layered above
satisfiability.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.