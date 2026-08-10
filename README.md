# tpt-formal

Formal-verification & design-by-contract foundation for TPT Solutions.

Dual-licensed under **MIT OR Apache-2.0**.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE-APACHE)

This repo consolidates TPT Solutions' formal-verification and design-by-contract
primitives into one dedicated pillar. The authoritative design notes live in
[`spec.txt`](spec.txt); the per-task tracker is [`todo.md`](todo.md).

## Pillar principles

- Per-crate `license = "MIT OR Apache-2.0"`.
- `no_std` where the underlying technique is usable in the embedded /
  safety-critical verticals that motivate this pillar (marked below).
- Many small crates plus a thin umbrella, not one broad crate.

## Crate table

| Crate | Phase | `no_std` | Internal deps | Purpose |
|-------|-------|----------|---------------|---------|
| `tpt-for-typestate` | 1 | yes | — | Phantom/ghost/newtype/bounded/safe-cast toolkit |
| `tpt-for-assert-const` | 1 | yes | — | Compile-time assertions |
| `tpt-for-contract` | 2 | yes | — | Pre/post/invariant/loop-invariant (self-contained; `tpt-math-numeric` composition documented) |
| `tpt-for-witness` | 3 | yes | tpt-for-typestate | Witness types |
| `tpt-for-refinement` | 3 | yes | tpt-for-contract | Refinement types |
| `tpt-for-verified-algorithms` | 4 | no | tpt-for-contract | Verified algorithm implementations (gcd/clamp/binary-search/insertion-sort) |
| `tpt-for-verified-ode` | 4 | no | tpt-for-contract, tpt-sci-ode | Verified ODE solving — **deferred** (needs cross-repo `tpt-sci-ode`) |
| `tpt-for-smt-lite` | 5 | no | — | Lightweight SMT bridge: term/problem builder + SMT-LIB2 + minimal evaluator (ADR 0007: `rsmt2`/`z3` valid wrap targets) |
| `tpt-for-proof-ast` | 6 | no | — | Proof AST representation |
| `tpt-for-det-proptest` | 6 | no | — | Deterministic property-based testing |
| `tpt-for-deterministic-sim` | 6 | no | — | Deterministic simulation harness |
| `tpt-for-redundancy` | 6 | yes | — | Redundancy/fault-tolerance primitives |
| `tpt-for-trace-macros` | 6 | yes | — | Trace/instrumentation macros |

## Status

Phases 0–6 are largely landed. `tpt-for-contract`, `tpt-for-witness`,
`tpt-for-refinement`, `tpt-for-verified-algorithms`, and `tpt-for-smt-lite` are
implemented and tested; the `no_std` crates build for `thumbv6m-none-eabi`.
`tpt-for-verified-ode` remains **deferred** — it needs the cross-repo
`tpt-sci-ode` (sibling `tpt-science` repo), which is not built yet. See
[`todo.md`](todo.md) for the per-phase tracker.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
