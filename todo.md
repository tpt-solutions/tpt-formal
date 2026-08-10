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
- [ ] Confirm all 13 `tpt-for-*` crates are in `tpt-rust-map/registry.toml`
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

**`tpt-for-verified-ode`** — verified ODE solving (needs `tpt-for-contract` + external `tpt-sci-ode`)
- [ ] Confirm `tpt-sci-ode` available as path/git dep — cross-repo
      prerequisite, `tpt-science` not yet built either
- [ ] Scaffold crate
- [ ] Design public API (how contract wrapping composes with `tpt-sci-ode`)
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

> DEFERRED: `tpt-sci-ode` does not exist yet in the sibling `tpt-science` repo.

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

## Phase 7 — Cross-Crate Integration & Workspace QA

- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean
- [ ] `cargo fmt --check` clean
- [ ] `cargo deny check` passes (licenses/duplicates/advisories)
- [ ] no_std verification for every no_std-marked crate (build against
      `thumbv6m-none-eabi`, per `rust-toolchain.toml` targets)
- [ ] Cross-crate integration tests for realistic combinations (e.g.
      `tpt-for-contract` + `tpt-for-refinement` together;
      `tpt-for-verified-ode` against real `tpt-sci-ode`)
- [ ] Root `README.md` finalized: full crate table + dependency graph

## Phase 8 — Release & Publish

do not publish unless explicitly asked for
- [ ] Version all 13 crates `0.1.0`, changelog entries
- [ ] Tag release; publish to crates.io in dependency order
- [ ] Flip `tpt-rust-map/registry.toml` status → `published` for all 13
      `tpt-for-*` entries
- [ ] Follow-up (tracked, not blocking): revisit ADR 0006's crate grouping
      once a real consumer (`tpt-flight-control`, `tpt-dynamo`, `tpt-vanguard`,
      `tpt-chassis`, `tpt-servo`, `tpt-relay`, etc.) depends on `tpt-formal`
      and reveals whether the 13-crate split matches actual usage
