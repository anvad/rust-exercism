use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    let expected_len = word.len();
    let word = word.to_lowercase();
    let mut word_cv = word.chars().collect::<Vec<char>>();
    word_cv.sort_unstable();
    possible_anagrams
        .iter()
        .filter(|pa| pa.len() == expected_len)
        .map(|pa| (pa, pa.to_lowercase())) // keep ref to original word
        .filter(|(_, pa)| *pa != word)
        .map(|(ow, pa)| {
            let mut pa_cv = pa.chars().collect::<Vec<char>>();
            pa_cv.sort_unstable();
            (*ow, pa_cv)
        })
        .filter(|(_, pa_cv)| *pa_cv == word_cv)
        .for_each(|(ow, _)| {
            anagrams.insert(ow);
        });
    anagrams
}
