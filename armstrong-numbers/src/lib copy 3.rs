pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = vec![];
    let mut my_num = num;
    while my_num > 0 {
        digits.push(my_num % 10);
        my_num = my_num / 10;
    }
    let exp = digits.len() as u32;
    let sum = digits.iter().fold(Some(0u32), |sum, d| {
        d.checked_pow(exp)
            .and_then(|part| sum.and_then(|sum| sum.checked_add(part)))
    });
    match sum {
        Some(sum) => sum == num,
        _ => false,
    }
}
