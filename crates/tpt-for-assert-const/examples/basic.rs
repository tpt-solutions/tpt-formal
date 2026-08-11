//! Example: compile-time assertions that evaluate a Boolean condition.
use tpt_for_assert_const::const_assert;

fn main() {
    // Evaluated at compile time; a false condition would fail to build.
    const_assert!(true);
    const_assert!(std::mem::size_of::<u32>() == 4);
    println!("const assertions passed at compile time");
}
