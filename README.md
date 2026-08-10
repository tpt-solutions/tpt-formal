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
| `tpt-for-contract` | 2 | yes | tpt-math-numeric | Pre/post/invariant/loop-invariant |
| `tpt-for-witness` | 3 | yes | tpt-for-typestate | Witness types |
| `tpt-for-refinement` | 3 | yes | tpt-for-contract | Refinement types |
| `tpt-for-verified-algorithms` | 4 | no | tpt-for-contract | Verified algorithm implementations |
| `tpt-for-verified-ode` | 4 | no | tpt-for-contract, tpt-sci-ode | Verified ODE solving |
| `tpt-for-smt-lite` | 5 | no | (external SMT binding) | Lightweight SMT bridge |
| `tpt-for-proof-ast` | 6 | no | — | Proof AST representation |
| `tpt-for-det-proptest` | 6 | no | — | Deterministic property-based testing |
| `tpt-for-deterministic-sim` | 6 | no | — | Deterministic simulation harness |
| `tpt-for-redundancy` | 6 | yes | — | Redundancy/fault-tolerance primitives |
| `tpt-for-trace-macros` | 6 | yes | — | Trace/instrumentation macros |

## Status

Phase 0 (bootstrap) and the no-internal-dependency crates are landing first.
Crates whose build requires a cross-repo dependency that is not yet published
(`tpt-math-numeric`, `tpt-sci-ode`) or an external-binding audit (`tpt-for-smt-lite`,
ADR 0007) are scaffolded once those prerequisites resolve — see [`todo.md`](todo.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
