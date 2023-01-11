pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = vec![];
    let mut my_num = num;
    while my_num > 0 {
        digits.push(my_num % 10);
        my_num = my_num / 10;
    }
    let sum = digits.iter().fold(0u32, |sum, d| {
        if let Some(part) = d.checked_pow(digits.len() as u32) {
            match sum.checked_add(part) {
                Some(sum) => sum,
                _ => 0,
            }
        } else {
            0
        }
    });
    println!("{num} = {:?}, sum = {}", digits, sum);
    num == sum as u32
}
