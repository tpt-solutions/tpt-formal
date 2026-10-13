//! D2Q9 lattice-Boltzmann BGK collision with a mass-conservation contract.
//!
//! # Theorem (exact arithmetic)
//!
//! With `ρ = Σ fᵢ`, `feqᵢ = wᵢ ρ (1 + 3 cᵢ·u + 4.5 (cᵢ·u)² − 1.5 |u|²)` and
//! `f'ᵢ = fᵢ − ω (fᵢ − feqᵢ)`, the lattice weights satisfy `Σ wᵢ = 1`,
//! `Σ wᵢ cᵢ = 0` and `Σ wᵢ (cᵢ·u)² = |u|²/3`, hence `Σ feqᵢ = ρ` and
//! `Σ f'ᵢ = Σ fᵢ − ω (ρ − ρ) = Σ fᵢ`.
//!
//! # Floating-point bound
//!
//! Every lane performs a bounded number of operations (about 30 per lane plus
//! two 9-term reductions), so by the standard `γₙ = nε/(1 − nε)` analysis the
//! computed drift obeys
//! `|Σ f'ᵢ − Σ fᵢ| ≤ MASS_K · ε · (Σ|fᵢ| + Σ Aᵢ)` with
//! `Aᵢ = wᵢ ρ (1 + 3|cᵢ·u| + 4.5 (cᵢ·u)² + 1.5 |u|²) ≥ |feqᵢ|`, `ε = 2⁻⁵³` and
//! `ω < 2`. [`MASS_K`] is a deliberately conservative constant; the tests
//! confirm the observed drift is far below it.

use tpt_for_contract::{ensures, requires};

/// Lattice velocities `cᵢ` of D2Q9.
pub const C: [(i32, i32); 9] = [
    (0, 0),
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
];

/// Lattice weights `wᵢ` of D2Q9 (sum to 1).
pub const W: [f64; 9] = [
    4.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 36.0,
    1.0 / 36.0,
    1.0 / 36.0,
    1.0 / 36.0,
];

/// Unit roundoff `ε = 2⁻⁵³` for `f64` with round-to-nearest.
pub const EPS: f64 = f64::EPSILON / 2.0;

/// Conservative constant in the mass-drift bound (see module docs).
pub const MASS_K: f64 = 64.0;

/// Result of a checked collision: mass before/after and the allowed drift.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassCheck {
    /// `Σ fᵢ` before the collision.
    pub before: f64,
    /// `Σ f'ᵢ` after the collision.
    pub after: f64,
    /// The bound `MASS_K · ε · scale` that `|after − before|` must respect.
    pub bound: f64,
}

/// Density `ρ = Σ fᵢ`.
pub fn density(f: &[f64; 9]) -> f64 {
    f.iter().sum()
}

/// Sum of densities over a lattice.
pub fn total_mass(cells: &[[f64; 9]]) -> f64 {
    cells.iter().map(density).sum()
}

/// Second-order equilibrium populations for density `rho` and velocity `(ux, uy)`.
pub fn equilibrium(rho: f64, ux: f64, uy: f64) -> [f64; 9] {
    let usq = ux * ux + uy * uy;
    let mut feq = [0.0; 9];
    for ((out, &(cx, cy)), &w) in feq.iter_mut().zip(C.iter()).zip(W.iter()) {
        let cu = f64::from(cx) * ux + f64::from(cy) * uy;
        *out = w * rho * (1.0 + 3.0 * cu + 4.5 * cu * cu - 1.5 * usq);
    }
    feq
}

/// Absolute-value scale `Σ|fᵢ| + Σ Aᵢ` used by the drift bound.
fn scale(f: &[f64; 9], rho: f64, ux: f64, uy: f64) -> f64 {
    let usq = ux * ux + uy * uy;
    let mut s: f64 = f.iter().map(|x| x.abs()).sum();
    for (&(cx, cy), &w) in C.iter().zip(W.iter()) {
        let cu = f64::from(cx) * ux + f64::from(cy) * uy;
        s += w * rho * (1.0 + 3.0 * cu.abs() + 4.5 * cu * cu + 1.5 * usq);
    }
    s
}

/// The allowed `f64` mass drift for a collision of `f` (precondition: `Σ f > 0`).
pub fn mass_error_bound(f: &[f64; 9]) -> f64 {
    let rho = density(f);
    let (ux, uy) = velocity(f, rho);
    MASS_K * EPS * scale(f, rho, ux, uy)
}

fn velocity(f: &[f64; 9], rho: f64) -> (f64, f64) {
    let (mut jx, mut jy) = (0.0, 0.0);
    for (&fi, &(cx, cy)) in f.iter().zip(C.iter()) {
        jx += fi * f64::from(cx);
        jy += fi * f64::from(cy);
    }
    (jx / rho, jy / rho)
}

/// Apply one BGK collision in place and check that mass is conserved.
///
/// # Panics
///
/// Panics (contract violation) if `ω ∉ (0, 2)`, any population is non-finite,
/// the density is not strictly positive, or the mass drift exceeds
/// [`mass_error_bound`].
pub fn collide_bgk(f: &mut [f64; 9], omega: f64) -> MassCheck {
    requires!(
        omega.is_finite() && omega > 0.0 && omega < 2.0,
        "omega must lie in (0, 2)"
    );
    requires!(
        f.iter().all(|x| x.is_finite()),
        "populations must be finite"
    );
    let rho = density(f);
    requires!(rho.is_finite() && rho > 0.0, "density must be positive");

    let (ux, uy) = velocity(f, rho);
    let bound = MASS_K * EPS * scale(f, rho, ux, uy);
    let feq = equilibrium(rho, ux, uy);
    for (fi, &e) in f.iter_mut().zip(feq.iter()) {
        *fi -= omega * (*fi - e);
    }
    let after = density(f);
    ensures!(
        (after - rho).abs() <= bound,
        "collision must conserve mass within the f64 bound"
    );
    MassCheck {
        before: rho,
        after,
        bound,
    }
}

/// Collide every cell of a lattice; returns the worst per-cell drift seen.
pub fn collide_all(cells: &mut [[f64; 9]], omega: f64) -> f64 {
    cells
        .iter_mut()
        .map(|f| {
            let m = collide_bgk(f, omega);
            (m.after - m.before).abs()
        })
        .fold(0.0, f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights_sum_to_one() {
        assert!((W.iter().sum::<f64>() - 1.0).abs() <= 4.0 * EPS);
    }

    #[test]
    fn equilibrium_density_matches() {
        let feq = equilibrium(1.3, 0.05, -0.02);
        assert!((density(&feq) - 1.3).abs() <= 1e-14);
    }

    #[test]
    fn collision_conserves_mass() {
        let mut f = [0.5, 0.11, 0.12, 0.13, 0.14, 0.03, 0.02, 0.04, 0.01];
        let m = collide_bgk(&mut f, 1.2);
        assert!((m.after - m.before).abs() <= m.bound);
        assert!(m.bound < 1e-13);
    }

    #[test]
    fn lattice_total_mass_is_preserved() {
        let mut cells = vec![[0.4, 0.1, 0.1, 0.1, 0.1, 0.03, 0.03, 0.03, 0.03]; 16];
        let before = total_mass(&cells);
        collide_all(&mut cells, 1.7);
        assert!((total_mass(&cells) - before).abs() <= 1e-13);
    }

    #[test]
    #[should_panic]
    fn rejects_omega_out_of_range() {
        collide_bgk(&mut [1.0 / 9.0; 9], 2.0);
    }

    #[test]
    #[should_panic]
    fn rejects_nonpositive_density() {
        collide_bgk(&mut [0.0; 9], 1.0);
    }
}
