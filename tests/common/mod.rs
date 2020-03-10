pub fn is_even(v: u32) -> bool {
    (v % 2) == 0
}

pub fn is_power_of_2(v: u32) -> bool {
    v > 0 && (v & (v - 1) == 0)
}
