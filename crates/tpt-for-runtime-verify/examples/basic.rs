fn main() {
    use tpt_for_runtime_verify::{Formula, Monitor, Step, Verdict};

    // Globally, every `req` is eventually followed by an `ack`:
    //   G (req -> F ack)
    let spec = Formula::globally(Formula::implies(
        Formula::atom("req"),
        Formula::eventually(Formula::atom("ack")),
    ));
    let mut mon = Monitor::new(spec);

    mon.observe(&Step::from_names(["req"]));
    assert_eq!(mon.verdict(), Verdict::Inconclusive); // ack not seen yet
    mon.observe(&Step::from_names(["ack"]));
    // The finite prefix is violation-free, but `G` can never be *confirmed* by
    // a finite prefix — the future could still break it.
    assert_eq!(mon.verdict(), Verdict::Inconclusive);
}
