// 192,000 ns/iter for test_factors_include_large_prime bench
// so while vs for iterator- not much diff
pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::<u64>::new();
    let maxf = (n as f64).sqrt() as u64 + 2;
    let mut f = 2;
    while f < maxf {
        while n % f == 0 {
            factors.push(f);
            n = n / f
        }
        f += 1;
    }
    if n != 1 {
        factors.push(n);
    }
    factors
}
