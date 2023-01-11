use std::collections::HashMap;
use std::{mem, thread};
pub fn frequency<'a>(input: &'a [&str], worker_count: usize) -> HashMap<char, usize> {
    let counter = |input: &[&str]| {
        let mut map = HashMap::new();
        for line in input {
            for c in line
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase())
            {
                *map.entry(c).or_default() += 1;
            }
        }
        map
    };
    // redirect to the best implementation.
    match input.len() {
        0 => HashMap::new(),
        n if n < 500 => counter(input),
        _ => {
            let mut handles = Vec::with_capacity(worker_count);
            for lines in input.chunks(input.len() / worker_count + 1) {
                // SAFETY: All threads will be joined right below,
                // thus it is guaranteed the shared references are OK.
                let input = unsafe { mem::transmute::<&'a [&str], &'static [&str]>(lines) };
                handles.push(thread::spawn(move || counter(input)))
            }
            let mut map = handles.pop().unwrap().join().unwrap();
            for res in handles {
                res.join().unwrap().into_iter().for_each(|(k, v)| {
                    *map.entry(k).or_default() += v;
                })
            }
            map
        }
    }
}
