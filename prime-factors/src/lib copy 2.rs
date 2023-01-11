// really slow!
pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::<u64>::new();
    println!("primes= {:?}", find_primes_till(n));
    for f in find_primes_till(n) {
        while n % f == 0 {
            factors.push(f);
            n = n / f
        }
        if n == 1 {
            break;
        }
    }
    factors
}

fn find_primes_till(n: u64) -> Vec<u64> {
    let n = n as usize;

    // using the Sieve of Eratosthenes method
    let mut nums = vec![true; n + 1];
    let max_i = (n as f64).sqrt() as usize;
    for i in 2..=n {
        if nums[i] == true {
            let mut j = i * i;
            while j < n {
                nums[j] = false;
                j += i;
            }
        }
    }
    nums.iter()
        .enumerate()
        .skip(2) // 0 and 1 are also set as true
        .filter(|&(_, is_prime)| *is_prime)
        .map(|(i, _)| i as u64)
        .collect()
}
