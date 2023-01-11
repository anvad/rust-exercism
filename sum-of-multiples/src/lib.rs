// copied rpearce's solution
//  removed extra `into_iter` and combined the 2nd filter into the any predicate
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|&n| factors.iter().any(|&f| f > 0 && n % f == 0))
        .sum()
}
