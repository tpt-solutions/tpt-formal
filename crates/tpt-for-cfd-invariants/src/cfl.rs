//! First-order upwind advection with a CFL-stability energy contract.
//!
//! # Theorem (exact arithmetic)
//!
//! For `u_t + a u_x = 0` (`a > 0`, periodic) the upwind update is
//! `uⱼ' = uⱼ − c (uⱼ − uⱼ₋₁) = (1 − c) uⱼ + c uⱼ₋₁` with `c = a Δt / Δx`.
//! If `0 < c < 1` this is a convex combination, so by convexity of `x ↦ x²`
//! `(uⱼ')² ≤ (1 − c) uⱼ² + c uⱼ₋₁²`. Summing over the periodic lattice gives
//! `Σ (uⱼ')² ≤ Σ uⱼ²`, i.e. the discrete energy `E = ½ Δx Σ uⱼ²` is
//! non-increasing and therefore bounded by `E₀` for every number of steps.
//! (`c ≤ 1` suffices mathematically; the contract is strict, `c < 1`.)
//!
//! # Floating-point slack
//!
//! Each update and the energy sum incur rounding, so the contract allows
//! `E' ≤ E · (1 + `[`slack`]`(n))` per step, with `slack(n) = 4 (n + 16) ε`.

use tpt_for_contract::{ensures, invariant, requires};

/// Unit roundoff `ε = 2⁻⁵³`.
pub const EPS: f64 = f64::EPSILON / 2.0;

/// Relative rounding slack allowed on the energy for an `n`-cell state.
pub fn slack(n: usize) -> f64 {
    4.0 * (n as f64 + 16.0) * EPS
}

/// CFL number `|a| Δt / Δx`.
///
/// # Panics
///
/// Panics if `dt` or `dx` is not finite and strictly positive, or `a` is not finite.
pub fn cfl_number(a: f64, dt: f64, dx: f64) -> f64 {
    requires!(a.is_finite(), "speed must be finite");
    requires!(dt.is_finite() && dt > 0.0, "dt must be positive");
    requires!(dx.is_finite() && dx > 0.0, "dx must be positive");
    a.abs() * dt / dx
}

/// Discrete energy `½ Δx Σ uⱼ²`.
pub fn energy(u: &[f64], dx: f64) -> f64 {
    0.5 * dx * u.iter().map(|x| x * x).sum::<f64>()
}

/// Unchecked periodic upwind update (shared with the negative-control test).
fn upwind_raw(u: &[f64], c: f64) -> Vec<f64> {
    let n = u.len();
    (0..n)
        .map(|j| u[j] - c * (u[j] - u[(j + n - 1) % n]))
        .collect()
}

/// One periodic upwind step, checked against the energy contract.
///
/// # Panics
///
/// Panics if `u` is empty or non-finite, `c ∉ (0, 1)`, or the energy grows
/// beyond the rounding slack.
pub fn upwind_step(u: &[f64], c: f64) -> Vec<f64> {
    requires!(!u.is_empty(), "state must be non-empty");
    requires!(u.iter().all(|x| x.is_finite()), "state must be finite");
    requires!(
        c.is_finite() && c > 0.0 && c < 1.0,
        "CFL number must lie in (0, 1)"
    );
    let next = upwind_raw(u, c);
    let (e0, e1) = (energy(u, 1.0), energy(&next, 1.0));
    ensures!(
        e1 <= e0 * (1.0 + slack(u.len())),
        "CFL < 1 must not grow the energy"
    );
    next
}

/// Advance `steps` upwind steps; returns the final state and the energy history
/// (`steps + 1` entries, starting with `E₀`).
pub fn run(u0: &[f64], c: f64, steps: usize, dx: f64) -> (Vec<f64>, Vec<f64>) {
    let mut u = u0.to_vec();
    let mut history = vec![energy(&u, dx)];
    for _ in 0..steps {
        u = upwind_step(&u, c);
        let e = energy(&u, dx);
        let prev = history[history.len() - 1];
        invariant!(e <= prev * (1.0 + slack(u.len())));
        history.push(e);
    }
    (u, history)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfl_number_basic() {
        assert!((cfl_number(2.0, 0.1, 0.5) - 0.4).abs() < 1e-15);
    }

    #[test]
    fn energy_decays_below_one() {
        let u0: Vec<f64> = (0..64).map(|j| if j < 32 { 1.0 } else { 0.0 }).collect();
        let (_, h) = run(&u0, 0.5, 200, 1.0);
        assert!(h.last().unwrap() < &h[0]);
        assert!(h.windows(2).all(|w| w[1] <= w[0] * (1.0 + slack(64))));
    }

    #[test]
    #[should_panic]
    fn rejects_cfl_at_or_above_one() {
        upwind_step(&[1.0, 0.0, 1.0], 1.0);
    }

    #[test]
    #[should_panic]
    fn rejects_nonpositive_cfl() {
        upwind_step(&[1.0, 0.0, 1.0], 0.0);
    }

    #[test]
    fn negative_control_energy_grows_above_one() {
        let mut u: Vec<f64> = (0..32)
            .map(|j| if j % 2 == 0 { 1.0 } else { -1.0 })
            .collect();
        let e0 = energy(&u, 1.0);
        for _ in 0..20 {
            u = upwind_raw(&u, 1.5);
        }
        assert!(energy(&u, 1.0) > 100.0 * e0);
    }
}
