// stevensonmt's solution
pub fn nth(x: u32) -> u64 {
    sieve(x + 1)
}
fn sieve(a: u32) -> u64 {
    let x = a as f64;
    let upper_limit = (x * ((x * x.ln()).ln())).floor() as u64;
    (2..upper_limit)
        .into_iter()
        .filter(|&n| prime(n))
        .take(x as usize)
        .last()
        .unwrap_or(a as u64 + 1) // only have None if limit was <=2, which is only true for 1st and 2nd primes which are conveniently 2 and 3 respectively
}
fn prime(n: u64) -> bool {
    match n {
        2 => true,
        x if x % 2 == 0 => false,
        x => prime2(x, 3),
    }
}
fn prime2(n: u64, k: u64) -> bool {
    match (n, k) {
        (a, b) if a < b * b => true,
        (a, b) if a % b == 0 => false,
        (a, b) => prime2(a, b + 2),
    }
}
