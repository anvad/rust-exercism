/// Check a Luhn checksum.
/// mix of nickclaw's and JaneL's solutions
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|ch| *ch != ' ')
        .rev()
        .try_fold((0, 0), |(len, sum), ch| match (len % 2, ch.to_digit(10)) {
            (0, Some(n)) => Ok((len + 1, sum + n)),
            (1, Some(n)) if n > 4 => Ok((len + 1, sum + 2 * n - 9)),
            (1, Some(n)) => Ok((len + 1, sum + 2 * n)),
            (_, _) => Err("digit conversion failed"),
        })
        .map_or(false, |(len, sum)| len > 1 && sum % 10 == 0)
}
