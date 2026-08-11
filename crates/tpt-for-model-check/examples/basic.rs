fn main() {
    use tpt_for_model_check::{check_safety, Model, SafetyResult};

    // A bounded counter that must stay below 3.
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct Counter(u32);
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    enum Act {
        Inc,
    }

    impl Model for Counter {
        type State = Counter;
        type Action = Act;
        fn initials(&self) -> Vec<Counter> {
            vec![Counter(0)]
        }
        fn actions(&self, s: &Counter) -> Vec<Act> {
            if s.0 < 2 {
                vec![Act::Inc]
            } else {
                vec![]
            }
        }
        fn step(&self, s: &Counter, _: &Act) -> Counter {
            Counter(s.0 + 1)
        }
        fn is_error(&self, s: &Counter) -> bool {
            s.0 >= 3
        }
    }

    let r = check_safety(&Counter(0));
    assert!(matches!(r, SafetyResult::Safe));
}
