/// possible errors for algebraic notation conversion
#[derive(Debug, PartialEq)]
pub enum Errors {
    NonAlphabeticFirstChar,
    NonNumericLastChar,
}

/// Converts chess algebraic notation to a coord
/// # Arguments
/// * `an` - the string of algebraic notation e.g. 'g5'
pub fn an_to_coord(an: &str) -> Result<(usize, usize), Errors> {
    let lower = an.to_lowercase();
    let mut iter = lower.chars();
    let first = match iter.next() {
        Some(first) if first.is_alphabetic() => first,
        _ => return Err(Errors::NonAlphabeticFirstChar),
    };
    let last = match iter.next() {
        Some(last) if last.is_numeric() => last,
        _ => return Err(Errors::NonNumericLastChar),
    };
    Ok((first as usize - 97, 8 - last.to_digit(10).unwrap() as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_an_to_coord() {
        assert_eq!(an_to_coord("a1").unwrap(), (0, 7));
        assert_eq!(an_to_coord("h6").unwrap(), (7, 2));
        assert_eq!(an_to_coord("g5").unwrap(), (6, 3));
        assert_eq!(an_to_coord("42"), Err(Errors::NonAlphabeticFirstChar));
        assert_eq!(an_to_coord("g"), Err(Errors::NonNumericLastChar));
    }
}