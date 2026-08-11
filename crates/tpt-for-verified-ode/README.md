# tpt-for-verified-ode

Verified ODE solving: contract-guarded fixed-step integrators.

Fixed-step initial-value-problem integrators whose pre-/post-/loop-invariants
are enforced by `tpt-for-contract`. The solvers are generic over an `OdeSystem`
trait, so any first-order system `y' = f(t, y)` can be solved without a heavier
numeric stack.

## Features

- `OdeSystem` — describe a first-order system `dy/dt = f(t, y)` by its
  dimension and right-hand side.
- `solve_euler(...)` — explicit Euler integration with contract-checked inputs.
- `solve_rk4(...)` — classical 4th-order Runge–Kutta integration.
- Every solver requires `dt > 0`, `t_end > t0`, and `y0.len() == sys.dim()`,
  and guarantees a non-empty trajectory starting at `(t0, y0)`.

## Example

```rust
use tpt_for_verified_ode::{OdeSystem, solve_rk4};

// Exponential decay: y' = -y, y(0) = 1  →  y(t) = e^{-t}
struct Decay;
impl OdeSystem for Decay {
    fn dim(&self) -> usize { 1 }
    fn rhs(&self, _t: f64, y: &[f64], dydt: &mut [f64]) {
        dydt[0] = -y[0];
    }
}

let traj = solve_rk4(&Decay, &[1.0], 0.0, 1.0, 0.1);
let final_y = traj.last().unwrap().y[0];
assert!((final_y - (-1.0_f64).exp()).abs() < 1e-4);
```

## Cargo features

No optional features. Depends on `tpt-for-contract`.

## Integration

Self-contained, contract-guarded integrators today; the spec pairs this crate
with `tpt-science`'s `tpt-sci-ode` for adaptive methods once that crate is
published, behind the same `OdeSystem` surface.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.