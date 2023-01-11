// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;

// surfingtomchen's solution is great!

fn get_count_hashmap<'a, T>(items: &'a [&T]) -> HashMap<&'a T, u32>
where
    T: Eq + Hash + ?Sized,
{
    items
        .iter()
        .fold(HashMap::new(), |mut count_hashmap, item| {
            *count_hashmap.entry(item).or_insert(0) += 1;
            count_hashmap
        })
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let available_words = get_count_hashmap(magazine);
    let needed_words = get_count_hashmap(note);
    needed_words
        .iter()
        .all(|(w, count)| available_words.get(w).unwrap_or(&0) >= count)
}
