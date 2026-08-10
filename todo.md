# tpt-formal — Project TODO

Formal-verification & design-by-contract foundation for TPT Solutions.
Dual-licensed MIT OR Apache-2.0. Tracks against `spec.txt` and
`tpt-rust-map/registry.toml`. Legacy `tpt-zero-formal`/`tpt-formal-lab` no
longer exist on disk — every crate below is a from-scratch build guided by
spec.txt's per-crate build notes, not a code port.

## Phase 0 — Repo Bootstrap

- [ ] `git init`, initial commit
- [ ] Copy `Cargo.toml`, `rustfmt.toml`, `deny.toml`, `rust-toolchain.toml`,
      `.github/workflows/ci.yml`, `LICENSE-MIT`, `LICENSE-APACHE` from
      `tpt-rust-map/template/`
- [ ] Fill workspace `Cargo.toml` `[workspace.package]`: `repository`/`homepage`
      = tpt-solutions/tpt-formal, `license = "MIT OR Apache-2.0"`,
      `edition = "2021"`, `rust-version = "1.75"`; leave `members` list to grow
      as crates land
- [ ] Add root `README.md`: crate map/table, license badges, link to `spec.txt`
- [ ] Confirm CI `no_std` job in `ci.yml` is updated from its placeholder to
      explicit `-p <crate>` flags once the first no_std crate lands (per ADR
      0001 — no blind `--workspace` no_std build)
- [ ] Confirm all 13 `tpt-for-*` crates are in `tpt-rust-map/registry.toml`
      (already done, `status = "planned"`) — flip each to `status = "git"`
      once this repo is pushed to a remote

## Phase 1 — Type-Level Safety Toolkit (no internal deps)

**`tpt-for-typestate`** [no_std] — phantom/ghost/newtype/bounded/safe-cast toolkit
- [ ] Scaffold crate (`Cargo.toml` w/ `license.workspace = true`, `src/lib.rs`, README)
- [ ] Design public API surface (module layout for the 6 merged facets)
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

**`tpt-for-assert-const`** [no_std] — compile-time assertions
- [ ] Scaffold crate
- [ ] Design public API (macro surface for const assertions)
- [ ] Implement
- [ ] Unit tests (incl. compile-fail tests via `trybuild` or similar)
- [ ] Docs + examples

## Phase 2 — Design-by-Contract Core

**`tpt-for-contract`** [no_std] — pre/post/invariant/loop-invariant
- [ ] Confirm `tpt-math-numeric` available as path/git dep (pre-publish) or
      crates.io dep (post-publish) — cross-repo prerequisite, `tpt-math` not
      yet built either
- [ ] Scaffold crate
- [ ] Design public API (unify precond/postcond/invariant/loop-inv into one
      coherent contract surface)
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

## Phase 3 — Contract/Typestate Extensions

**`tpt-for-witness`** [no_std] — witness types (needs `tpt-for-typestate`)
- [ ] Scaffold crate
- [ ] Design public API
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

**`tpt-for-refinement`** [no_std] — refinement types (needs `tpt-for-contract`)
- [ ] Scaffold crate
- [ ] Design public API
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

## Phase 4 — Verified Numerics & Algorithms

**`tpt-for-verified-algorithms`** — verified algorithm implementations (needs `tpt-for-contract`)
- [ ] Scaffold crate
- [ ] Design public API (which algorithm families ship first)
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

**`tpt-for-verified-ode`** — verified ODE solving (needs `tpt-for-contract` + external `tpt-sci-ode`)
- [ ] Confirm `tpt-sci-ode` available as path/git dep — cross-repo
      prerequisite, `tpt-science` not yet built either
- [ ] Scaffold crate
- [ ] Design public API (how contract wrapping composes with `tpt-sci-ode`)
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

## Phase 5 — SMT Bridge

**`tpt-for-smt-lite`** — lightweight SMT bridge
- [ ] Audit existing Rust SMT-solver bindings for a valid wrap target — license
      MUST be `MIT OR Apache-2.0` or more permissive per ADR 0007
      (Apache-2.0-only disqualifies, no exceptions)
- [ ] If a valid binding exists: scaffold as a thin wrapper. If not: document
      the "build, not wrap" decision and reason
- [ ] Scaffold crate
- [ ] Design public API (bridge surface)
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

## Phase 6 — Proof, Testing, Simulation & Fault-Tolerance Harnesses

(All independent — no internal deps, any order, parallelizable)

**`tpt-for-proof-ast`** — proof AST representation
- [ ] Scaffold crate
- [ ] Design public API
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

**`tpt-for-det-proptest`** — deterministic property-based testing
- [ ] Scaffold crate
- [ ] Design public API
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

**`tpt-for-deterministic-sim`** — deterministic simulation harness
- [ ] Scaffold crate
- [ ] Design public API
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

**`tpt-for-redundancy`** [no_std] — redundancy/fault-tolerance primitives
- [ ] Scaffold crate
- [ ] Design public API
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

**`tpt-for-trace-macros`** [no_std] — trace/instrumentation macros
- [ ] Scaffold crate
- [ ] Design public API (macro surface)
- [ ] Implement
- [ ] Unit tests
- [ ] Docs + examples

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

- [ ] Version all 13 crates `0.1.0`, changelog entries
- [ ] Tag release; publish to crates.io in dependency order
- [ ] Flip `tpt-rust-map/registry.toml` status → `published` for all 13
      `tpt-for-*` entries
- [ ] Follow-up (tracked, not blocking): revisit ADR 0006's crate grouping
      once a real consumer (`tpt-flight-control`, `tpt-dynamo`, `tpt-vanguard`,
      `tpt-chassis`, `tpt-servo`, `tpt-relay`, etc.) depends on `tpt-formal`
      and reveals whether the 13-crate split matches actual usage
