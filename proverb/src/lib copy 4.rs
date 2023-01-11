// implemented slighly modified version of nickers's solution

// learnt the difference between `.iter()` and `.into_iter()`
//  learnt about `std::iter::once` even though i did not use it
pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => "".to_string(),
        Some(word) => {
            list.windows(2)
                .map(|slice| format!("For want of a {} the {} was lost.\n", slice[0], slice[1]))
                .collect::<String>()
                + &format!("And all for the want of a {word}.")
        }
    }
}
