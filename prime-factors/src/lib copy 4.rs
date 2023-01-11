// 564,000 ns/iter for test_factors_include_large_prime bench
pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    match (2..n + 1).find(|x| n % x == 0) {
        Some(x) => {
            result.push(x);
            result.append(&mut factors(n / x));
        }
        None => {}
    }
    return result;
}
