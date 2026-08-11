//! Example: verify a small program with a generated verification condition.
use tpt_for_vcgen::{verify, BExpr, Expr, Spec, Stmt, VcResult};

fn main() {
    // x := 1; x := x + 1;   post: x == 2
    let spec = Spec {
        pre: BExpr::bool(true),
        post: BExpr::eq(Expr::var("x"), Expr::const_(2)),
    };
    let body = vec![
        Stmt::assign("x", Expr::const_(1)),
        Stmt::assign("x", Expr::add(Expr::var("x"), Expr::const_(1))),
    ];
    let result = verify(&spec, &body);
    println!("verified = {}", matches!(result, VcResult::Verified));
}
