mod common;
use common::*;
use mulch;

#[test]
fn test_power_of_2() {
    assert!(is_power_of_2(mulch::get_4()))
}
