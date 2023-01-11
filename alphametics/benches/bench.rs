#![feature(test)]
extern crate test;
use test::Bencher;

use alphametics;

// 686k ns/iter for flesmes
// 2.2m ns/iter for v2.5 with v2.4 + cols_map vector + u32 hashmap
// 2.4m ns/iter for v2.5 with v2.4 + cols_map vector
// 2.7m ns/iter for v2.4 with v2.3 + col_char_count vector
// 4.3m ns/iter for v2.3 with 2.1 and no need to use col_nums vector
// 6.8m ns/iter for v2.1 with deterministic chars array, with .rev()
// 4.5m ns/iter for v2.1 with deterministic chars array, without .rev()
// 8.1m ns/iter for v2
#[bench]
fn test_puzzle_with_ten_letters(b: &mut Bencher) {
    b.iter(|| alphametics::solve("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE"));
}

// 475k ns/iter for v2.1 with deterministic chars array, with .rev()
// 1.2m ns/iter for v2.1 with deterministic chars array, without .rev()
// 23m  ns/iter for v2.1 with deterministic chars array, without .rev() and no reversed()
// 855k ns/iter for v2
#[bench]
fn test_puzzle_with_eight_letters(b: &mut Bencher) {
    b.iter(|| alphametics::solve("SEND + MORE == MONEY"));
}

// 104k ns/iter for v1
#[bench]
fn test_puzzle_with_four_letters(b: &mut Bencher) {
    b.iter(|| alphametics::solve("AS + A == MOM"));
}

// 3.4m ns/iter for v1
// 55k  ns/iter for v2
#[bench]
fn test_puzzle_with_six_letters(b: &mut Bencher) {
    b.iter(|| alphametics::solve("NO + NO + TOO == LATE"));
}
