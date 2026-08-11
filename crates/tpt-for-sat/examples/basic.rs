fn main() {
    use tpt_for_sat::{Cnf, SatResult, Solver};

    // (x) ∧ (¬x ∨ y) ∧ (¬y ∨ z) ∧ (¬z)  → UNSAT (x, y, z all forced false/true)
    let cnf = Cnf::from_lits(3, &[&[1], &[-1, 2], &[-2, 3], &[-3]]).unwrap();
    let mut solver = Solver::new(cnf);
    assert_eq!(solver.solve(), SatResult::Unsat);
}
