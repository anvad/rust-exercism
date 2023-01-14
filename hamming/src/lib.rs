/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    // since these are DNA strands, all the chars are ascii chars, so
    //  `bytes()` is cheaper than `chars()`
    Some(
        s1.bytes()
            .zip(s2.bytes())
            .filter(|(c1, c2)| *c1 != *c2)
            .count(),
    )
}
