fn main() {
    use tpt_for_symbolic_exec::{run, SExpr, SStmt, ViolationKind};

    // z = 10 / x;   with x a free symbolic input → x == 0 is feasible.
    let prog = vec![SStmt::assign(
        "z",
        SExpr::div(SExpr::const_(10), SExpr::sym("x")),
    )];
    let report = run(&prog);
    assert_eq!(report.violations.len(), 1);
    assert!(matches!(
        report.violations[0].kind,
        ViolationKind::DivByZero
    ));
}
