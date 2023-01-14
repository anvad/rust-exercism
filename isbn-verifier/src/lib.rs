/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: Vec<char> = isbn.chars().filter(|ch| *ch != '-').collect();
    isbn.len() == 10
        && isbn[..isbn.len() - 1]
            .iter()
            .all(|ch| *ch >= '0' && *ch <= '9')
        && (*isbn.last().unwrap() == 'X' || isbn.last().unwrap().to_digit(10).is_some())
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
