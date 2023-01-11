/*
test bench_large_parallel   ... bench:     139,955 ns/iter (+/- 7,918)
test bench_large_sequential ... bench:     244,454 ns/iter (+/- 4,004)
test bench_small_parallel   ... bench:      52,084 ns/iter (+/- 4,802)
test bench_small_sequential ... bench:       8,671 ns/iter (+/- 206)
test bench_tiny_parallel    ... bench:      18,045 ns/iter (+/- 1,264)
test bench_tiny_sequential  ... bench:          39 ns/iter (+/- 1)
 */

use std::{
    collections::HashMap,
    thread::{self, ScopedJoinHandle},
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if worker_count == 1 {
        return frequency_seq(input);
    }
    let mut hm = HashMap::new();
    let num_lines = input.len();
    let chunk_size = match num_lines % worker_count {
        0 => num_lines / worker_count,
        _ => num_lines / worker_count + 1,
    };
    if chunk_size == 0 {
        return hm;
    }

    thread::scope(|s| {
        let mut handles: Vec<ScopedJoinHandle<HashMap<char, usize>>> = vec![];
        for chunk in input.chunks(chunk_size) {
            handles.push(s.spawn(|| frequency_seq(chunk)));
        }
        for handle in handles {
            let thread_hm = handle.join().unwrap();
            for (&key, &ti) in thread_hm.iter() {
                hm.entry(key).and_modify(|i| *i += ti).or_insert(ti);
            }
        }
    });

    hm
}

pub fn frequency_seq(input: &[&str]) -> HashMap<char, usize> {
    let mut hm = HashMap::new();
    for line in input {
        for ch in line.chars().filter(|ch| ch.is_alphabetic()) {
            if let Some(c) = ch.to_lowercase().next() {
                hm.entry(c).and_modify(|i| *i += 1).or_insert(1);
            }
        }
    }
    hm
}
