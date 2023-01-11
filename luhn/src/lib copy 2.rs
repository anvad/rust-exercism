/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: Vec<_> = code.as_bytes().iter().filter(|&&ch| ch != b' ').collect();
    if code.len() < 2 {
        return false;
    }
    code.iter()
        .rev()
        .enumerate()
        .try_fold(0u8, |luhn, (i, &ch)| {
            if (*ch < b'0') || (*ch > b'0' + 9) {
                return Err("only digits and spaces allowed");
            }
            if (i % 2) == 0 || *ch == b'9' {
                // all the digits that must be added without changing
                Ok(luhn + *ch - b'0')
            } else {
                // all the digits that must be doubled before adding
                Ok(luhn + (*ch - b'0') * 2 % 9)
            }
        })
        .unwrap_or(1) // unwrap failure means this is not valid luhn
        % 10
        == 0
}
