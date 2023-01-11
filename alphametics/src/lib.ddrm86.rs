// ddrm86's solution
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
fn parse(input: &str) -> (Vec<String>, HashSet<char>) {
    let split_addends_sum: Vec<&str> = input.split("==").collect();
    let mut split: Vec<&str> = split_addends_sum.get(0).unwrap().split('+').collect();
    split.push(split_addends_sum.get(1).unwrap());
    let parsed: Vec<String> = split.iter().map(|s| s.trim().to_string()).collect();
    let not_zero: HashSet<char> = parsed.iter().map(|s| s.chars().next().unwrap()).collect();
    (parsed, not_zero)
}
fn to_number(string: &str, solution: &HashMap<char, u8>) -> u64 {
    let num_str: String = string
        .chars()
        .map(|c| solution.get(&c).unwrap().to_string())
        .collect();
    num_str
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Something happened while parsing: {}", num_str))
}
fn check(alphametic: &[String], not_zero: &HashSet<char>, solution: &HashMap<char, u8>) -> bool {
    if not_zero.iter().any(|c| solution.get(c).unwrap() == &0) {
        return false;
    }
    let sum: Vec<u64> = alphametic.iter().map(|s| to_number(s, solution)).collect();
    sum[0..sum.len() - 1].iter().sum::<u64>() == sum[sum.len() - 1]
}
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let charset: HashSet<char> = input.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let (alphametic, not_zero) = parse(input);
    let perms = (0..=9).permutations(charset.len());
    for perm in perms {
        let solution: HashMap<char, u8> = charset.iter().cloned().zip(perm.into_iter()).collect();
        if check(&alphametic, &not_zero, &solution) {
            return Some(solution);
        }
    }
    None
}
