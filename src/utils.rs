use std::cmp;

pub fn clamp(num: i32, min: i32, max: i32) -> i32 {
    cmp::min(cmp::max(num, min), max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(10, 1, 7), 7);
        assert_eq!(clamp(-1, 0, 5), 0);
    }
}
