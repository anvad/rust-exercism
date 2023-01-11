// slowbyrne's solution is also nice! copied it below
// see how we take the slice given by window and convert it to
//  a tuple for pattern matching
pub fn abbreviate(phrase: &str) -> String {
    " ".chars()
        .chain(phrase.chars())
        .collect::<Vec<_>>()
        .windows(2)
        .map(|window| match (window[0], window[1]) {
            (c1, c2) if " -_".contains(c1) && c2.is_alphabetic() => c2.to_ascii_uppercase(),
            (c1, c2) if c1.is_lowercase() && c2.is_uppercase() => c2,
            _ => ' ',
        })
        .filter(|ch| !ch.is_ascii_whitespace())
        .collect()
}
