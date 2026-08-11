//! Example: integrate a simple first-order ODE system with forward Euler.
use tpt_for_verified_ode::{solve_euler, OdeSystem, Point};

struct Linear;
impl OdeSystem for Linear {
    fn dim(&self) -> usize {
        1
    }
    fn rhs(&self, _t: f64, y: &[f64], dydt: &mut [f64]) {
        dydt[0] = y[0];
    }
}

fn main() {
    let pts: Vec<Point> = solve_euler(&Linear, &[1.0], 0.0, 1.0, 0.1);
    println!(
        "integrated {} points; final t = {}",
        pts.len(),
        pts.last().unwrap().t
    );
}
