// slower at 35,000 ns/iter (vs 3,000 ns/iter) for test_factors_include_large_prime bench
// but eliminates while loops and is tail recursive
// run using `rustup run nightly cargo bench`
pub fn factors(n: u64) -> Vec<u64> {
    fi(n, 2, (n as f64).sqrt() as u64 + 1, Vec::<u64>::new())
}

fn fi(n: u64, f: u64, maxf: u64, mut factors: Vec<u64>) -> Vec<u64> {
    if f > maxf {
        if n > 1 {
            factors.push(n)
        }
        factors
    } else if n % f == 0 {
        factors.push(f);
        fi(n / f, f, (n as f64).sqrt() as u64, factors)
    } else {
        let step = match f {
            2 => 1,
            _ => 2,
        };
        fi(n, f + step, (n as f64).sqrt() as u64, factors)
    }
}
