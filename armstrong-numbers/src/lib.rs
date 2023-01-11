pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let exp = num_str.len() as u32;
    num_str
        .as_bytes()
        .iter()
        .map(|byte| (*byte - b'0') as u32)
        .try_fold(0u32, |sum, d| {
            d.checked_pow(exp).and_then(|part| sum.checked_add(part))
        })
        .map_or(false, |sum| sum == num)
}
