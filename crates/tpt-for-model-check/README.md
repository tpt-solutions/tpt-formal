# tpt-for-model-check

Explicit-state model checking (clean-room implementation).

Answers a *safety* question — "is a bad state reachable from an initial
state?" — by exploring the reachable state graph. When the property is violated
it returns a concrete counterexample trace, the diagnostic that makes model
checking useful in practice.

## Features

- `Model` — describe a (finite, reachable) transition system: `initials`,
  `actions`, `step`, and `is_error`.
- `check_safety(model)` — BFS over the state graph; returns `Safe` or a
  `Violated` counterexample (shortest witness trace).
- `reachable(model)` — enumerate every reachable state (coverage / debugging).
- Clean-room implementation; `stateright` (MIT) is documented as a possible future
  backend behind a feature flag, mirroring `tpt-for-smt-lite`'s handling of
  `rsmt2`/`z3`.

## Example

```rust
use tpt_for_model_check::{Model, check_safety, SafetyResult};

// A bounded counter that must stay below 3.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Counter(u32);
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Act { Inc }

impl Model for Counter {
    type State = Counter;
    type Action = Act;
    fn initials(&self) -> Vec<Counter> { vec![Counter(0)] }
    fn actions(&self, s: &Counter) -> Vec<Act> {
        if s.0 < 2 { vec![Act::Inc] } else { vec![] }
    }
    fn step(&self, s: &Counter, _: &Act) -> Counter { Counter(s.0 + 1) }
    fn is_error(&self, s: &Counter) -> bool { s.0 >= 3 }
}

let r = check_safety(&Counter(0));
assert!(matches!(r, SafetyResult::Safe));
```

## Cargo features

No optional features. Pure `std` (no external dependencies).

## Integration

The state-exploration engine for the workspace's verification story; pairs with
`tpt-for-proof-ast` (obligations) and `tpt-for-trace-macros` (trace capture).

## Status

This crate is in early (0.1.0) development. APIs may change between releases.

## License

Licensed under either of [MIT](https://opensource.org/licenses/MIT) or
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) at your option.