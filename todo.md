# tpt-formal — Project TODO

Formal-verification & design-by-contract foundation for TPT Solutions.
Dual-licensed MIT OR Apache-2.0. Tracks against `spec.txt` and
`tpt-rust-map/registry.toml`. Legacy `tpt-zero-formal`/`tpt-formal-lab` no
longer exist on disk — every crate below is a from-scratch build guided by
spec.txt's per-crate build notes, not a code port.

## Phase 0 — Repo Bootstrap

- [x] `git init` (initial commit deferred — not requested this pass)
- [x] Copy `Cargo.toml`, `rustfmt.toml`, `deny.toml`, `rust-toolchain.toml`,
      `.github/workflows/ci.yml`, `LICENSE-MIT`, `LICENSE-APACHE` from
      `tpt-rust-map/template/`
- [x] Fill workspace `Cargo.toml` `[workspace.package]`: `repository`/`homepage`
      = tpt-solutions/tpt-formal, `license = "MIT OR Apache-2.0"`,
      `edition = "2021"`, `rust-version = "1.75"`; leave `members` list to grow
      as crates land
- [x] Add root `README.md`: crate map/table, license badges, link to `spec.txt`
- [x] Confirm CI `no_std` job in `ci.yml` is updated from its placeholder to
      explicit `-p <crate>` flags once the first no_std crate landed (per ADR
      0001 — no blind `--workspace` no_std build)
- [ ] Confirm all 19 `tpt-for-*` crates are in `tpt-rust-map/registry.toml`
      (already done, `status = "planned"`) — flip each to `status = "git"`
      once this repo is pushed to a remote

## Phase 1 — Type-Level Safety Toolkit (no internal deps)

**`tpt-for-typestate`** [no_std] — phantom/ghost/newtype/bounded/safe-cast toolkit
- [x] Scaffold crate (`Cargo.toml` w/ `license.workspace = true`, `src/lib.rs`, README)
- [x] Design public API surface (module layout for the 6 merged facets)
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

**`tpt-for-assert-const`** [no_std] — compile-time assertions
- [x] Scaffold crate
- [x] Design public API (macro surface for const assertions)
- [x] Implement
- [x] Unit tests (compile-fail deferred — uses const-context assertions)
- [x] Docs + examples

## Phase 2 — Design-by-Contract Core

**`tpt-for-contract`** [no_std] — pre/post/invariant/loop-invariant
- [x] Implemented self-contained (no `tpt-math-numeric` hard dependency); the
      crate is generic over any `bool` condition, so `tpt-math-numeric` types
      compose directly. Wire the path/git dep when `tpt-math` is published.
- [x] Scaffold crate
- [x] Design public API (unify precond/postcond/invariant/loop-inv into one
      coherent contract surface via `requires!`/`ensures!`/`invariant!`/
      `loop_invariant!` + `Invariant` trait + `ContractError`)
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

## Phase 3 — Contract/Typestate Extensions

**`tpt-for-witness`** [no_std] — witness types (needs `tpt-for-typestate`)
- [x] Scaffold crate
- [x] Design public API
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

**`tpt-for-refinement`** [no_std] — refinement types (needs `tpt-for-contract`)
- [x] Scaffold crate
- [x] Design public API (`Refined<T, P>` + `Predicate`, blanket `Invariant` impl)
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

## Phase 4 — Verified Numerics & Algorithms

**`tpt-for-verified-algorithms`** — verified algorithm implementations (needs `tpt-for-contract`)
- [x] Scaffold crate
- [x] Design public API (gcd, clamp, binary-search, insertion-sort guarded by contracts)
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

**`tpt-for-verified-ode`** — verified ODE solving (needs `tpt-for-contract`)
- [x] Implemented self-contained: contract-guarded Euler/RK4 integrators over a
      generic `OdeSystem` trait. `tpt-sci-ode` is the designated high-performance
      backend to wrap later (documented; not yet published).
- [x] Scaffold crate
- [x] Design public API (`OdeSystem` trait + `solve_euler`/`solve_rk4`)
- [x] Implement
- [x] Unit tests (exponential-decay convergence vs e^{-t})
- [x] Docs + examples

> all 19 `tpt-for-*` crates are now implemented. `tpt-sci-ode` (sibling
> `tpt-science`) remains the only outstanding external dependency, tracked as a
> future backend for `tpt-for-verified-ode`.

## Phase 5 — SMT Bridge

**`tpt-for-smt-lite`** — lightweight SMT bridge
- [x] Audit existing Rust SMT-solver bindings (ADR 0007): `rsmt2` (MIT/Apache-2.0)
      and `z3` (MIT) are valid wrap targets (Apache-2.0-only disqualifies — none
      of the candidates violate this).
- [x] Scaffold crate: solver-agnostic term/problem builder + SMT-LIB2 serializer
      + minimal built-in ground evaluator (external solver wrap deferred to a
      `backend-rsmt2` feature pending solver binary in CI).
- [x] Design public API (bridge surface: `Sort`/`Term`/`Problem`/`check_sat`)
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

## Phase 6 — Proof, Testing, Simulation & Fault-Tolerance Harnesses

(All independent — no internal deps, any order, parallelizable)

**`tpt-for-proof-ast`** — proof AST representation
- [x] Scaffold crate
- [x] Design public API
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

**`tpt-for-det-proptest`** — deterministic property-based testing
- [x] Scaffold crate
- [x] Design public API
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

**`tpt-for-deterministic-sim`** — deterministic simulation harness
- [x] Scaffold crate
- [x] Design public API
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

**`tpt-for-redundancy`** [no_std] — redundancy/fault-tolerance primitives
- [x] Scaffold crate
- [x] Design public API
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

**`tpt-for-trace-macros`** [no_std] — trace/instrumentation macros
- [x] Scaffold crate
- [x] Design public API (macro surface)
- [x] Implement
- [x] Unit tests
- [x] Docs + examples

## Phase 7 — Ecosystem-Gap Crates (added after later spec.txt research pass)

Six crates with no legacy predecessor — new capability, not a consolidation
of `tpt-zero-formal`/`tpt-formal-lab` crates. All independent of each other;
`tpt-for-vcgen` needs `tpt-for-contract` + `tpt-for-smt-lite`, both already
implemented, so it can start immediately. None block Phase 8/9.

> **Phase 7 status:** all six crates are scaffolded, designed, implemented,
> tested, and documented. `cargo test --workspace`, `cargo clippy
> --workspace --all-targets --all-features -- -D warnings`, and `cargo fmt
> --check` all pass. The `tpt-rust-map/registry.toml` pre-check could not be
> performed (sibling repo unavailable in this environment). `model-check` is
> clean-room (a `stateright` external backend is documented, mirroring
> `smt-lite`'s `rsmt2`/`z3` notes); `runtime-verify` is clean-room per the
> `rtlola` licensing note.

**`tpt-for-model-check`** — explicit-state model checking (clean-room; `stateright` documented as external backend)
- [ ] Check `tpt-rust-map/registry.toml` for an existing entry (sibling repo
      unavailable in this environment — blocked)
- [x] Decide backend: clean-room implementation; `stateright` (MIT) documented
      as a future external backend behind a feature flag
- [x] Scaffold crate
- [x] Design public API (`Model` trait: `initials`/`actions`/`step`/`is_error`
      + BFS safety check returning a `Counterexample` on violation)
- [x] Implement (worklist BFS over the reachable state graph)
- [x] Unit tests (safe/unsafe bounded counter, mutual-exclusion violation)
- [x] Docs + examples

**`tpt-for-sat`** — from-scratch pure-Rust CDCL SAT solver, no FFI
- [ ] Check `tpt-rust-map/registry.toml` (sibling repo unavailable — blocked)
- [x] Scaffold crate
- [x] Design public API (`Cnf`/`Lit`/`Clause`/`Solver`/`SatResult`)
- [x] Implement CDCL core (watched literals, 1UIP clause learning, VSIDS, restarts)
- [x] Unit tests (SAT/UNSAT instances, unit propagation, learned clause)
- [x] Docs + examples

**`tpt-for-vcgen`** — verification-condition generation (needs `tpt-for-contract` + `tpt-for-smt-lite`, both implemented)
- [ ] Check `tpt-rust-map/registry.toml` (sibling repo unavailable — blocked)
- [x] Scaffold crate
- [x] Design public API (WP calculus over `Expr`/`BExpr`/`Stmt`/`Spec` ->
      `tpt-for-smt-lite` `Term`/`Problem`)
- [x] Implement (`wp` + `generate_vc` + `verify`)
- [x] Unit tests (ground verified/falsified programs, SMT-LIB2 serialization)
- [x] Docs + examples

**`tpt-for-abstract-interp`** — generic abstract-interpretation framework
- [ ] Check `tpt-rust-map/registry.toml` (sibling repo unavailable — blocked)
- [x] Scaffold crate
- [x] Design public API (`AbstractDomain` trait, `Interval` domain, `analyze` fixpoint)
- [x] Implement (widening-based worklist fixpoint over a CFG)
- [x] Unit tests (interval lattice, loop-bound `x in [10, +inf)`)
- [x] Docs + examples

**`tpt-for-symbolic-exec`** — whole-program symbolic execution
- [ ] Check `tpt-rust-map/registry.toml` (sibling repo unavailable — blocked)
- [x] Scaffold crate
- [x] Design public API (`SExpr`/`SCond`/`SStmt` + `run` returning `SymReport`)
- [x] Implement (symbolic store, path conditions, `tpt-for-smt-lite` backend)
- [x] Unit tests (div-by-zero detection, broken-assertion, branch pruning)
- [x] Docs + examples

**`tpt-for-runtime-verify`** — runtime verification / temporal-logic monitoring over live traces
- [ ] Check `tpt-rust-map/registry.toml` (sibling repo unavailable — blocked)
- [x] Scaffold crate
- [x] Design public API (`Formula` LTL fragment + `Monitor`/`Verdict`)
- [x] Implement (clean-room monitor, `Satisfied`/`Violated`/`Inconclusive`)
- [x] Unit tests (G/F/U/X semantics, incremental monitor)
- [x] Docs + examples

## Phase 8 — Cross-Crate Integration & Workspace QA

- [x] `cargo test --workspace` passes
- [x] `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean
- [x] `cargo fmt --check` clean
- [x] `cargo deny check` passes (licenses/duplicates/advisories)
- [x] no_std verification for every no_std-marked crate (build against
      `thumbv6m-none-eabi`, per `rust-toolchain.toml` targets)
- [x] Cross-crate integration tests for realistic combinations (contract +
      refinement; see `tpt-for-refinement/tests/integration.rs`)
- [x] Root `README.md` finalized: full crate table + dependency graph


## Phase 10 — Correctness/Security Audit Follow-Up (2026-08-11)

A full-workspace review found no literal stubs (`todo!()`/`unimplemented!()`), but a deep
read of the six Phase 7 crates plus a security audit surfaced real algorithmic bugs (some
masked by tests that assert the buggy behavior), one memory-safety bug, and several
panic-as-DoS entry points. Tracked here per-fix; see
`C:\Users\Phillip\.claude\plans\review-project-fix-any-nifty-meadow.md` for full detail.

**Correctness / soundness fixes**
- [x] `tpt-for-sat`: `propagate()`'s unit-propagation branch doesn't establish the
      "asserted literal at position 0 of its reason clause" invariant `analyze()` relies
      on — swap the implied literal into position 0 before enqueueing; add a regression
      test that forces the buggy code path
- [x] `tpt-for-abstract-interp`: `analyze()`'s "pull" recompute-from-predecessors
      unconditionally overwrites `state[node]`, which can shrink it since `widen` isn't
      monotonic in its first argument — remove the pull mechanism, rely solely on the
      incremental push/worklist propagation seeded at `[entry]`; add a diamond-merge test
- [x] `tpt-for-runtime-verify`: `Globally` returns `Satisfied` instead of `Inconclusive`
      for a finite prefix with no violation — fix to always return `None`; correct the
      `globally_violated` test assertion
- [x] `tpt-for-vcgen`: `wp(Assume(c), q)` uses `c ∧ q` instead of the correct `c ⟹ q` —
      fix to `BExpr::implies`; correct the `assume_introduces_guard` test to demonstrate
      the now-correct `Verified` result; fix the doc/README claim that vcgen depends on
      `tpt-for-contract` (it doesn't — only `tpt-for-smt-lite`)
- [x] `tpt-for-symbolic-exec`: (a) `exec`'s `If` arm unconditionally returns, dropping every
      statement after the `If` in the enclosing block — fix via indexed iteration +
      continuation-concatenation into each branch; (b) nested `Div` (e.g. `z = 10/x + 1`)
      is never checked for div-by-zero — add a recursive `collect_divs` walk; (c) using a
      division result in a later condition panics via `SExpr::to_term()` — change
      `to_term()` to return `Option<Term>`, treat `None` as conservatively feasible like
      `Unknown`. Add regression tests for all three.
- [x] `tpt-for-trace-macros`: `RingTrace`'s global tracer manufactures an aliased `&mut`
      from a raw pointer with `Relaxed` ordering and no synchronization — real UB/data
      race reachable from ordinary multithreaded code with zero caller `unsafe`. Redesign
      around a `std::sync::Mutex`-protected static; collapse `RingTrace::new().install()`
      into `RingTrace::install(capacity) -> &'static RingTrace`; update the ring test.

**Security-audit hardening (panic-as-DoS on malformed input)**
- [x] `tpt-for-sat`: `Cnf::from_lits` doesn't validate var indices / rejects DIMACS `0` via
      panic — validate inline, return `Option<Cnf>`
- [x] `tpt-for-smt-lite`: `Term::eval`'s raw `i64` arithmetic can overflow-panic (release
      profile has `overflow-checks = true`) — switch to `checked_*` ops, propagate `None`
- [x] `tpt-for-abstract-interp`: `Expr::eval`/`Cfg::transfer` index directly by a
      caller-supplied `VarId` with no bounds check — guard both read (fall back to `top`)
      and write (no-op) sites
- [x] `tpt-for-symbolic-exec`: covered by the `Option<Term>` fix above

**Innovative additions**
- [x] Wire `tpt-for-sat` into `tpt-for-smt-lite::Problem::check_sat()` as a sound,
      one-directional boolean-abstraction (Tseitin → CNF → CDCL) decision tier for
      formulas the ground evaluator can't decide — upgrades `Unknown → Unsat` only, never
      unsoundly downgrades. Automatically benefits `tpt-for-vcgen::verify()` and
      `tpt-for-symbolic-exec`'s `feasible()`. Add tests in all three crates.
- [x] Add `examples/basic.rs` to all 19 crates (adapt each crate's existing top-of-file
      doctest into a standalone runnable example)
- [x] Add a differential/property test for `tpt-for-sat` using `tpt-for-det-proptest`:
      random small CNFs checked against a brute-force truth-table oracle — direct
      regression guard for the conflict-analysis fix above
- [x] Add a `cargo audit` (RustSec) job to `.github/workflows/ci.yml` alongside the
      existing `cargo-deny` job

## Phase 9 — Release & Publish

do not publish unless explicitly asked for
- [ ] Version all 19 crates `0.1.0`, changelog entries
- [ ] Tag release; publish to crates.io in dependency order
- [ ] Flip `tpt-rust-map/registry.toml` status -> `published` for all 19
      `tpt-for-*` entries
- [ ] Follow-up (tracked, not blocking): revisit ADR 0006's crate grouping
      once a real consumer (`tpt-flight-control`, `tpt-dynamo`, `tpt-vanguard`,
      `tpt-chassis`, `tpt-servo`, `tpt-relay`, etc.) depends on `tpt-formal`
      and reveals whether the 19-crate split matches actual usage


