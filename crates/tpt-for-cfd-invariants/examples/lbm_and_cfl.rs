//! Example: LBM mass conservation and CFL-gated energy stability.
//!
//! Happy path: a D2Q9 collision conserves mass to rounding error, and upwind
//! advection at CFL 0.5 never increases the discrete energy. Failure path:
//! CFL 1.2 violates the stability precondition and the contract panics.

use tpt_for_cfd_invariants::{cfl_number, collide_bgk, run, upwind_step};

fn main() {
    let mut f = [0.5, 0.11, 0.12, 0.13, 0.14, 0.03, 0.02, 0.04, 0.01];
    let m = collide_bgk(&mut f, 1.2);
    println!("LBM collision (omega = 1.2):");
    println!("  mass before = {:.15}", m.before);
    println!("  mass after  = {:.15}", m.after);
    println!(
        "  drift       = {:.2e}  (bound {:.2e})",
        (m.after - m.before).abs(),
        m.bound
    );

    let c = cfl_number(1.0, 0.005, 0.01);
    let u0: Vec<f64> = (0..100).map(|j| if j < 50 { 1.0 } else { 0.0 }).collect();
    let (_, history) = run(&u0, c, 100, 0.01);
    println!("Upwind advection (CFL = {c}):");
    println!("  E_0 = {:.6}, E_100 = {:.6}", history[0], history[100]);

    let bad = cfl_number(1.0, 0.012, 0.01);
    let result = std::panic::catch_unwind(|| upwind_step(&[1.0, 0.0, 1.0], bad));
    println!("CFL = {bad}: contract rejected = {}", result.is_err());
}
