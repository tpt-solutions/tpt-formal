//! Deterministic property tests for the CFD invariants.

use tpt_for_cfd_invariants::lbm::{collide_bgk, density};
use tpt_for_cfd_invariants::{cfl::slack, energy, run};
use tpt_for_det_proptest::{assert_prop, DeterministicRng, Gen};

fn unit(rng: &mut DeterministicRng) -> f64 {
    (rng.next_u64() >> 11) as f64 / (1u64 << 53) as f64
}

#[derive(Debug, Clone)]
struct Collision {
    f: [f64; 9],
    omega: f64,
}

struct CollisionGen;
impl Gen<Collision> for CollisionGen {
    fn generate(&self, rng: &mut DeterministicRng) -> Collision {
        let mut f = [0.0; 9];
        for x in f.iter_mut() {
            *x = 1e-3 + unit(rng);
        }
        Collision {
            f,
            omega: 0.01 + 1.98 * unit(rng),
        }
    }
}

#[derive(Debug, Clone)]
struct Advection {
    u: Vec<f64>,
    c: f64,
}

struct AdvectionGen;
impl Gen<Advection> for AdvectionGen {
    fn generate(&self, rng: &mut DeterministicRng) -> Advection {
        let n = 3 + rng.gen_range(62) as usize;
        Advection {
            u: (0..n).map(|_| 2.0 * unit(rng) - 1.0).collect(),
            c: 0.001 + 0.998 * unit(rng),
        }
    }
}

#[test]
fn collision_conserves_mass_within_bound() {
    assert_prop(&CollisionGen, 2000, 1, |mut case: Collision| {
        let before = density(&case.f);
        let m = collide_bgk(&mut case.f, case.omega);
        (m.after - before).abs() <= m.bound
    });
}

#[test]
fn energy_never_grows_below_cfl_one() {
    assert_prop(&AdvectionGen, 500, 2, |case: Advection| {
        let (_, h) = run(&case.u, case.c, 50, 1.0);
        let n = case.u.len();
        h.windows(2).all(|w| w[1] <= w[0] * (1.0 + slack(n)))
            && h[50] <= energy(&case.u, 1.0) * (1.0 + 50.0 * slack(n))
    });
}
