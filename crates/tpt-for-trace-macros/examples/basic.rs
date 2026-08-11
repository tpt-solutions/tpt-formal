//! Example: install a `RingTrace` collector and emit a few trace events.
use tpt_for_trace_macros::{trace_event, trace_value, uninstall, RingTrace};

fn main() {
    let ring = RingTrace::install(16);
    trace_event!("demo", "hello {}", "world");
    trace_value!("demo", [1, 2, 3]);
    println!("captured {} records", ring.len());
    uninstall();
}
