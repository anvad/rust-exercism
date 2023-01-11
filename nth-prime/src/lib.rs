// following https://www.baeldung.com/cs/prime-number-algorithms
// and https://math.stackexchange.com/questions/1257/is-there-a-known-mathematical-equation-to-find-the-nth-prime

// briankung's solution is really elegant.. i liked how he used an infinite iterator and used nth() to break out
//  i incorporated nth() in my code :)
//  my method is not as clean but runs much faster because the underlying algo is fast
//  my code returns in 80ms or less while his takes approx. 1s for 10,000th prime
//  `time cargo test test_big_prime -- --include-ignored`
pub fn nth(n: u32) -> u32 {
    let n = n as usize;

    // the nth prime (𝑝𝑛) will be found before `max` defined below
    // 𝑝𝑛 < 𝑛 * (ln(𝑛) + 𝑛 * ln(ln(𝑛))) for 𝑛 ≥ 6
    //  this method does not work for small values of n, so we are adding 13
    //  to cover the first 6 primes
    // we end up calculating a few more primes than needed
    //  for example, for n = 10,000, we calculate 10,454 primes before stopping!
    let max = 13 + n * (f64::ln(n as f64) + f64::ln(f64::ln(n as f64))) as usize;

    // using the Sieve of Eratosthenes method
    let mut nums = vec![true; max as usize];
    let max_i = f64::sqrt(max as f64) as usize;
    for i in 2..=max_i {
        if nums[i] == true {
            let mut j = i * i;
            while j < max {
                nums[j] = false;
                j += i;
            }
        }
    }
    nums.iter()
        .enumerate()
        .skip(2) // 0 and 1 are also set as true
        .filter(|&(_, is_prime)| *is_prime)
        .map(|(i, _)| i)
        .nth(n as usize)
        .unwrap_or(0) as u32
}
