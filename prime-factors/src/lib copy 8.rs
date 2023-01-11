// 3,000 ns/iter for test_factors_include_large_prime bench
pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::<u64>::new();
    while n % 2 == 0 {
        factors.push(2);
        n = n / 2;
    }
    let mut maxf = (n as f64).sqrt() as u64 + 1;
    let mut f = 3;
    while f <= maxf {
        while n % f == 0 {
            factors.push(f);
            n = n / f;
        }
        maxf = (n as f64).sqrt() as u64 + 1;
        f += 2;
    }
    if n != 1 {
        factors.push(n);
    }
    factors
}
