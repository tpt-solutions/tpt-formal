# tpt-for-trace-macros

Trace / instrumentation macros for verification (`no_std` global tracer).

A single global tracer function receives formatted trace lines produced by the
`trace_event!`, `trace_point!`, and `trace_value!` macros. The tracer is a plain
function pointer, so it works in `no_std` (install it from your HAL or runtime).
A `std`-gated `RingTrace` collector is provided for capturing traces in tests
and tooling.

## Features

- `set_tracer` / `tracer` / `emit` — install and invoke a global
  `fn(target, msg)` tracer.
- `trace_event!(target, ...)` — formatted trace line (rendered into a fixed
  `TRACE_BUF_LEN` buffer; longer messages are truncated).
- `trace_point!(target)` / `trace_value!(target, value)` — zero-arg and
  `Debug`-style helpers.
- `ArrayWriter` — a no-allocation `fmt::Write` sink backed by a byte buffer.
- `RingTrace` *(requires the `std` feature)* — a fixed-capacity ring buffer that
  installs itself as the global tracer and records `(seq, target, msg)`.

## Example

```rust
use tpt_for_trace_macros::{set_tracer, trace_event, trace_value};

// Install a tracer (in no_std, do this from your runtime/HAL).
set_tracer(|target, msg| println!("[{target}] {msg}"));
trace_event!("net", "packet received, len = {}", 42);
trace_value!("state", [1, 2, 3]);

// For tests/tooling, capture into a ring buffer (needs the `std` feature):
#[cfg(feature = "std")]
{
    use tpt_for_trace_macros::{RingTrace, uninstall};
    let ring = RingTrace::new(8).install();
    trace_event!("demo", "value = {}", 42);
    assert_eq!(ring.records()[0].msg, "value = 42");
    uninstall();
}
```

## Cargo features

- `std` *(default)* — enables the `RingTrace` collector and `Record` type.
- `alloc` — declared for downstream convenience; the core tracer is
  allocation-free and works in bare `no_std`.

## Integration

A lightweight observation layer used by the verification crates to emit and
capture execution traces during analysis and testing.

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.