// hmuendel's solution
// chekcs if a number can be factored into given prime factors
fn is_prime(n: u32, prime_factors: &Vec<u32>) -> bool {
    prime_factors
        .iter()
        // we don't check even numbers so we don't need to check 2
        .skip(1)
        // only checking until sqrt(n)
        .take_while(|&p| p * p <= n)
        // is prime if no of the given primes is a factor
        .all(|&p| n % p != 0)
}
pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::with_capacity(n as usize);
    primes.push(2);
    let mut candidate: u32 = 3;
    while primes.len() <= n as usize {
        if is_prime(candidate, &primes) {
            primes.push(candidate)
        }
        // we don't need to check even numbers
        candidate += 2;
    }
    //returning the nth found prime
    primes[n as usize]
}
