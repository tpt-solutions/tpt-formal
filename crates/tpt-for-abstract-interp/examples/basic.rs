fn main() {
    use tpt_for_abstract_interp::{analyze, AbstractDomain, Cfg, CmpOp, Expr, Interval, Stmt};

    // x = 0; while (x < 10) { x = x + 1; }   →  x ∈ [10, +∞) at exit
    let mut cfg = Cfg::new(1);
    cfg.block(&[Stmt::assign(0, Expr::const_(0))], &[1]);
    cfg.block(&[], &[2, 3]); // loop head
    cfg.block(
        &[
            Stmt::assume(0, CmpOp::Lt, 10),
            Stmt::assign(0, Expr::add(Expr::var(0), Expr::const_(1))),
        ],
        &[1],
    );
    cfg.block(&[Stmt::assume(0, CmpOp::Ge, 10)], &[4]);
    cfg.block(&[], &[]); // exit

    let states = analyze(&cfg, 0, vec![Interval::bottom()]).unwrap();
    let exit = states[4][0];
    assert_eq!(exit.lo(), 10);
    assert_eq!(exit.hi(), i64::MAX);
}
