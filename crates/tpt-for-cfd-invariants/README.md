# tpt-for-cfd-invariants

CFD invariants as checked contracts, built on `tpt-for-contract`.

| Invariant | Statement | Enforcement |
|-----------|-----------|-------------|
| LBM mass conservation | `Σ f'ᵢ = Σ fᵢ` for D2Q9 BGK collision | `ensures!` with bound `MASS_K · ε · scale` |
| CFL stability | `0 < CFL < 1` ⇒ upwind energy `½ΔxΣuⱼ²` is non-increasing | `requires!` on CFL, `ensures!`/`invariant!` on energy |

## Proof boundary

The exact-arithmetic statements are theorems argued in the module docs
(`lbm`, `cfl`). The `f64` implementation is checked at runtime against explicit
rounding bounds and exercised by deterministic property tests; this is **not**
a machine-checked proof. CFL < 1 gives energy stability for the upwind scheme
only — it is not a general guarantee for every discretisation.

```text
cargo run -p tpt-for-cfd-invariants --example cfd_invariants_basic
```
