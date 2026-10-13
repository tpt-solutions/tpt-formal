//! CFD invariants as checked contracts.
//!
//! Two invariants, each stated as an exact-arithmetic theorem and enforced in
//! `f64` with an explicit, documented error bound:
//!
//! 1. **Mass conservation** ([`lbm`]): the D2Q9 BGK collision step leaves the
//!    sum of populations (the density) unchanged. In real arithmetic the
//!    equality is exact; in `f64` the drift is bounded by
//!    [`lbm::MASS_K`]` · ε · scale`.
//! 2. **Stability** ([`cfl`]): for first-order upwind advection with
//!    `0 < CFL < 1`, each update is a convex combination of neighbours, so the
//!    discrete energy `½ Δx Σ uⱼ²` cannot grow, for any number of steps.
//!
//! These are runtime-checked contracts plus analytic bounds, not
//! machine-checked proofs: the theorems are argued in the module docs and the
//! `f64` claims are enforced by `ensures!` / `invariant!` and property tests.

pub mod cfl;
pub mod lbm;

pub use cfl::{cfl_number, energy, run, upwind_step};
pub use lbm::{
    collide_all, collide_bgk, density, equilibrium, mass_error_bound, total_mass, MassCheck,
};
