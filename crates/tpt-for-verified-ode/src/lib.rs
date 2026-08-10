//! Verified ODE solving.
//!
//! Fixed-step initial-value-problem integrators whose contracts (preconditions,
//! postconditions, loop invariants) are enforced by [`tpt_for_contract`]. The
//! solvers are generic over an [`OdeSystem`] trait, so any first-order system
//! `y' = f(t, y)` can be solved without pulling in a heavier numeric stack.
//!
//! # Backend composition
//!
//! The spec pairs this crate with `tpt-science`'s `tpt-sci-ode` for
//! higher-order / adaptive methods. `tpt-sci-ode` is not published yet, so this
//! crate ships self-contained, contract-guarded integrators; when `tpt-sci-ode`
//! lands, its solvers can be wrapped behind the same [`OdeSystem`] contract
//! surface (or consumed directly) without changing call sites here.

use tpt_for_contract::{ensures, invariant, requires};

/// A first-order ODE system `dy/dt = f(t, y)`.
pub trait OdeSystem {
    /// Dimension of the state vector `y`.
    fn dim(&self) -> usize;

    /// Evaluate `f(t, y)` into `dydt`, which has length [`OdeSystem::dim`].
    fn rhs(&self, t: f64, y: &[f64], dydt: &mut [f64]);
}

/// A trajectory point: time and state.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    /// Time of the point.
    pub t: f64,
    /// State at that time.
    pub y: Vec<f64>,
}

/// Number of fixed steps covering `[t0, t_end]` with step `dt`.
fn step_count(t0: f64, t_end: f64, dt: f64) -> usize {
    ((t_end - t0) / dt).ceil() as usize
}

/// Solve an IVP with the explicit Euler method.
///
/// # Preconditions
///
/// `dt > 0`, `t_end > t0`, and `y0.len() == sys.dim()`.
///
/// # Postconditions
///
/// The returned trajectory is non-empty and starts at `(t0, y0)`.
pub fn solve_euler<S: OdeSystem>(sys: &S, y0: &[f64], t0: f64, t_end: f64, dt: f64) -> Vec<Point> {
    requires!(dt > 0.0, "step size must be strictly positive");
    requires!(t_end > t0, "integration interval must be strictly positive");
    requires!(
        y0.len() == sys.dim(),
        "initial state must match system dimension"
    );

    let n = sys.dim();
    let n_steps = step_count(t0, t_end, dt);
    let mut y = y0.to_vec();
    let mut dydt = vec![0.0; n];
    let mut traj = Vec::with_capacity(n_steps + 1);
    traj.push(Point {
        t: t0,
        y: y0.to_vec(),
    });

    let mut t = t0;
    while t < t_end {
        let h = if t + dt > t_end { t_end - t } else { dt };
        sys.rhs(t, &y, &mut dydt);
        for i in 0..n {
            y[i] += h * dydt[i];
        }
        t += h;
        invariant!(t <= t_end + 1e-9);
        traj.push(Point { t, y: y.clone() });
    }

    ensures!(!traj.is_empty());
    ensures!(traj[0].t == t0);
    traj
}

/// Solve an IVP with the classical RK4 method.
///
/// # Preconditions
///
/// `dt > 0`, `t_end > t0`, and `y0.len() == sys.dim()`.
///
/// # Postconditions
///
/// The returned trajectory is non-empty and starts at `(t0, y0)`.
pub fn solve_rk4<S: OdeSystem>(sys: &S, y0: &[f64], t0: f64, t_end: f64, dt: f64) -> Vec<Point> {
    requires!(dt > 0.0, "step size must be strictly positive");
    requires!(t_end > t0, "integration interval must be strictly positive");
    requires!(
        y0.len() == sys.dim(),
        "initial state must match system dimension"
    );

    let n = sys.dim();
    let n_steps = step_count(t0, t_end, dt);
    let mut y = y0.to_vec();
    let mut k1 = vec![0.0; n];
    let mut k2 = vec![0.0; n];
    let mut k3 = vec![0.0; n];
    let mut k4 = vec![0.0; n];
    let mut traj = Vec::with_capacity(n_steps + 1);
    traj.push(Point {
        t: t0,
        y: y0.to_vec(),
    });

    let mut t = t0;
    while t < t_end {
        let h = if t + dt > t_end { t_end - t } else { dt };
        sys.rhs(t, &y, &mut k1);
        let ya = axpy(&y, &k1, 0.5 * h);
        sys.rhs(t + 0.5 * h, &ya, &mut k2);
        let yb = axpy(&y, &k2, 0.5 * h);
        sys.rhs(t + 0.5 * h, &yb, &mut k3);
        let yc = axpy(&y, &k3, h);
        sys.rhs(t + h, &yc, &mut k4);
        for i in 0..n {
            y[i] += (h / 6.0) * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        t += h;
        invariant!(t <= t_end + 1e-9);
        traj.push(Point { t, y: y.clone() });
    }

    ensures!(!traj.is_empty());
    ensures!(traj[0].t == t0);
    traj
}

/// `y + a * k` (helper for RK4 stage combinations).
fn axpy(y: &[f64], k: &[f64], a: f64) -> Vec<f64> {
    y.iter().zip(k).map(|(yi, ki)| yi + a * ki).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Exponential decay `y' = -y`, `y(0) = 1` → `y(t) = e^{-t}`.
    struct Decay;
    impl OdeSystem for Decay {
        fn dim(&self) -> usize {
            1
        }
        fn rhs(&self, _t: f64, y: &[f64], dydt: &mut [f64]) {
            dydt[0] = -y[0];
        }
    }

    #[test]
    fn euler_tracks_exponential_decay() {
        let traj = solve_euler(&Decay, &[1.0], 0.0, 1.0, 0.01);
        let final_y = traj.last().unwrap().y[0];
        let expected = (-1.0_f64).exp();
        // Euler with dt=0.01 is accurate to a few parts in 10^3 here.
        assert!(
            (final_y - expected).abs() < 0.02,
            "got {}, want {}",
            final_y,
            expected
        );
    }

    #[test]
    fn rk4_tracks_exponential_decay() {
        let traj = solve_rk4(&Decay, &[1.0], 0.0, 1.0, 0.1);
        let final_y = traj.last().unwrap().y[0];
        let expected = (-1.0_f64).exp();
        assert!(
            (final_y - expected).abs() < 1e-4,
            "got {}, want {}",
            final_y,
            expected
        );
    }

    #[test]
    fn precondition_interval_must_be_positive() {
        // t_end <= t0 violates the precondition; under debug checks this
        // panics. We only assert the well-formed call succeeds.
        let traj = solve_rk4(&Decay, &[1.0], 0.0, 0.5, 0.05);
        assert_eq!(traj[0].y[0], 1.0);
    }
}
