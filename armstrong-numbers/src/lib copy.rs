pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u128> = vec![];
    let mut my_num = num;
    while my_num > 0 {
        digits.push((my_num % 10) as u128);
        my_num = my_num / 10;
    }
    let sum = digits
        .iter()
        .fold(0u128, |sum, d| sum + d.pow(digits.len() as u32));
    println!("{num} = {:?}, sum = {}", digits, sum);
    if sum > (u32::MAX as u128) {
        return false;
    }
    num == sum as u32
}
