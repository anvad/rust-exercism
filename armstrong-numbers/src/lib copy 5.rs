pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    match num_str.as_bytes().iter().fold(Some(0u32), |sum, d| {
        ((*d - b'0') as u32)
            .checked_pow(num_str.len() as u32)
            .and_then(|part| sum.and_then(|sum| sum.checked_add(part)))
    }) {
        Some(sum) => sum == num,
        _ => false,
    }
}
