//! Example: redundancy voting — repeated independent computations are reconciled.
use tpt_for_redundancy::majority_vote;

fn main() {
    let votes = [1, 2, 2, 2, 3];
    println!("majority of {:?} = {:?}", votes, majority_vote(&votes));
}
