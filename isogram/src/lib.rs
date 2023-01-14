use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    match candidate
        .chars()
        .filter(|&ch| ch != '-' && ch != ' ')
        .map(|ch| ch.to_lowercase().to_string())
        .try_fold(HashMap::new(), |mut map, ch| match map.get(&ch) {
            Some(_) => None,
            _ => {
                map.insert(ch, 1);
                Some(map)
            }
        }) {
        Some(_) => true,
        _ => false,
    }
}
