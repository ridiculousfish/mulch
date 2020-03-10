mod common;
use common::*;
use mulch;

#[test]
fn test_even() {
    assert!(is_even(mulch::get_4()))
}
