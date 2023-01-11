use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    let expected_len = word.len();
    let word = word.to_lowercase();
    let mut word_cv = word.chars().collect::<Vec<char>>();
    word_cv.sort_unstable();
    possible_anagrams
        .iter()
        .enumerate()
        .filter(|(_, pa)| pa.len() == expected_len)
        .map(|(pos, pa)| (pos, (*pa).to_lowercase()))
        .filter(|(_, pa)| word != *pa)
        .map(|(pos, pa)| {
            let mut pa_cv = (*pa).chars().collect::<Vec<char>>();
            pa_cv.sort_unstable();
            (pos, pa_cv)
        })
        .filter(|(_, pw_cv)| *pw_cv == word_cv)
        .for_each(|(pos, _)| {
            anagrams.insert(possible_anagrams[pos]);
        });

    anagrams
}
