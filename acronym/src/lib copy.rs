pub fn abbreviate(phrase: &str) -> String {
    let phrase = phrase.replace("-", " ").replace("_", " ");
    let phrases = phrase.split(" ").collect::<Vec<_>>();
    match phrases[..] {
        [""] => String::new(),
        [first, ..] if first.to_uppercase() == first && first.chars().last() == Some(':') => {
            first[..first.len() - 1].to_string()
        }
        _ => phrases
            .iter()
            .filter(|word| !word.is_empty())
            .fold(Vec::<char>::new(), |mut acronym, word| {
                let mut chars = word.chars();
                let first = chars.nth(0).unwrap().to_ascii_uppercase();
                acronym.push(first);
                if *word == word.to_ascii_uppercase() {
                    return acronym;
                }
                for ch in chars.filter(|ch| ch.is_ascii_uppercase()) {
                    acronym.push(ch);
                }
                acronym
            })
            .iter()
            .collect(),
    }
}
