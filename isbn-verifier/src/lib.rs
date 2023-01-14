/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let is_digit = |ch: &char| *ch >= '0' && *ch <= '9';
    let last_char = |s: &[char]| s.last().unwrap().clone();
    let isbn: Vec<char> = isbn.chars().filter(|ch| *ch != '-').collect();
    isbn.len() == 10
        && isbn[..isbn.len() - 1].iter().all(is_digit)
        && (last_char(&isbn) == 'X' || is_digit(&last_char(&isbn)))
        && isbn
            .iter()
            .enumerate()
            .map(|(i, ch)| match ch {
                'X' => 10 * (10 - i as u32),
                _ => ch.to_digit(10).unwrap() * (10 - i as u32),
            })
            .sum::<u32>()
            % 11
            == 0
}
