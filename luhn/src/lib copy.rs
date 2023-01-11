/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: Vec<_> = code.as_bytes().iter().filter(|&&ch| ch != b' ').collect();
    if code.len() < 2 {
        return false;
    }
    let mut luhn = 0;
    for (i, &ch) in code.iter().rev().enumerate() {
        if (*ch < b'0') || (*ch > b'0' + 9) {
            return false;
        }
        if (i % 2) == 0 || *ch == b'9' {
            // all the digits that must be added without changing
            luhn += *ch - b'0';
        } else {
            // all the digits that must be doubled before adding
            luhn += (*ch - b'0') * 2 % 9;
        }
    }
    luhn % 10 == 0
}
