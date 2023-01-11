pub fn is_armstrong_number(num: u32) -> bool {
    let exp = num.to_string().len() as u32;
    let mut my_num = num;
    let mut sum = Some(0u32);
    while sum.is_some() && my_num > 0 {
        sum = (my_num % 10)
            .checked_pow(exp)
            .and_then(|part| sum.and_then(|sum| sum.checked_add(part)));
        my_num = my_num / 10;
    }
    match sum {
        Some(sum) => sum == num,
        _ => false,
    }
}
