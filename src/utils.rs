use std::cmp;

pub fn clamp(num: i32, min: i32, max: i32) -> i32 {
    cmp::min(cmp::max(num, min), max)
}
