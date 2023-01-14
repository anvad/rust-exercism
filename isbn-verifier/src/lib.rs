// my (previous) solution had multiple passes of the supplied string
// elshize's solution (reproduced below) is so much more elegant - with just one pass

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    match isbn
        .chars()
        .filter(|ch| *ch != '-')
        .enumerate()
        .map(|(pos, ch)| match (pos, ch) {
            (p, n) if n.is_ascii_digit() => Some((p, (n as u8 - b'0') as usize * (10 - p))),
            (9, 'X') => Some((9, 10)),
            _ => None,
        })
        .try_fold((0, 0), |(_, sum), elem| elem.map(|(p, n)| (p, sum + n)))
    {
        Some((9, sum)) => sum % 11 == 0,
        _ => false,
    }
}
