//! Example: build a simple proof AST and inspect its shape.
use tpt_for_proof_ast::{Formula, Proof, Term};

fn main() {
    let f = Formula::pred("even", vec![Term::var("n")]);
    let proof = Proof::axiom("even-zero", f);
    println!("proof depth = {}, size = {}", proof.depth(), proof.size());
}
