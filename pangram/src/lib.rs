use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut alphabet = HashSet::<char>::new();
    for ch in sentence
        .bytes()
        .filter(|ch| (*ch as char).is_ascii_alphabetic())
        .map(|ch| (ch as char).to_ascii_lowercase())
    {
        if alphabet.insert(ch) && alphabet.len() == 26 {
            return true;
        }
    }
    false
}
