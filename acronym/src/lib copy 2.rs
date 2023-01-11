// jtmueller's solution is so much more elegant. Copying it with minor modifications below.
pub fn abbreviate(phrase: &str) -> String {
    let is_upper = char::is_ascii_uppercase;
    phrase
        .split(|ch| ch == ' ' || ch == '-' || ch == '_')
        .flat_map(|word| {
            word.chars()
                .take(1)
                .map(|ch| ch.to_ascii_uppercase())
                .chain(word.chars().skip_while(is_upper).filter(is_upper))
        })
        .collect::<String>()
}
