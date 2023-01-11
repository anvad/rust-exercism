/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: Vec<_> = code.chars().filter(|&ch| ch != ' ').collect();
    if code.len() < 2 {
        return false;
    }
    code.iter()
        .rev()
        .enumerate()
        .try_fold(0u8, |luhn, (i, ch)| {
            // now iterating on utf-8 code points rather than assuming i 
            //  only receive ascii text
            if !(ch.is_ascii()) || (*ch < '0') || (*ch > '9') {
                println!("bad char: {ch}");
                return Err("only digits and spaces allowed");
            }
            if (i % 2) == 0 || *ch == '9' {
                // all the digits that must be added without changing
                // println!("ch: {ch}, new luhn asis: {}", (luhn + *ch as u8 - b'0') % 10);
                Ok((luhn + *ch as u8 - b'0') % 10)
            } else {
                // all the digits that must be doubled before adding
                // println!("ch: {ch}, new luhn  dbl: {}", (luhn + (*ch as u8 - b'0') * 2 % 9) % 10);
                Ok((luhn + (*ch as u8 - b'0') * 2 % 9) % 10)
            }
        })
        .unwrap_or(1) // unwrap failure means this is not valid luhn
        == 0
}
