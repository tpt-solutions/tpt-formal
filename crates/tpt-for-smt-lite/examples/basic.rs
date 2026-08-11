//! Example: build a small SMT problem and check satisfiability.
use tpt_for_smt_lite::{Problem, Sort, Term};

fn main() {
    let mut p = Problem::new();
    p.declare_const("x", Sort::Int);
    p.assert(Term::var("x").gt(Term::int(0)));
    println!("check_sat(x > 0) = {:?}", p.check_sat());
}
