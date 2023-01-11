// dguerri's solution

use std::collections::{HashMap, HashSet};
// Backtrack recursive algorithm
fn solve_inner(
    coefficients: &HashMap<char, i64>,
    initials_len: usize,
    vars: &[char],
    vals: &[u8],
    partial_sum: i64,
) -> Option<HashMap<char, u8>> {
    if vals.len() == vars.len() {
        if partial_sum == 0 {
            let solution: HashMap<char, u8> = vars
                .iter()
                .zip(vals.iter())
                .map(|(c, v)| (*c, *v))
                .collect();
            return Some(solution);
        }
        return None;
    }
    for n in (0..=9).filter(|n| !(vals.contains(n) || vals.len() < initials_len && *n == 0)) {
        if let Some(solution) = solve_inner(
            coefficients,
            initials_len,
            vars,
            &[vals, &[n]].concat(),
            partial_sum + *coefficients.get(&vars[vals.len()]).unwrap() * n as i64,
        ) {
            return Some(solution);
        }
    }
    None
}
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let op: Vec<&str> = input.split(" == ").collect();
    let terms: Vec<&str> = op[0].split(" + ").collect();
    let mut coefficients: HashMap<char, i64> = HashMap::new();
    for &term in terms.iter() {
        let len = term.len() - 1;
        for (i, c) in term.chars().enumerate() {
            *coefficients.entry(c).or_default() += 10_i64.pow((len - i) as u32);
        }
    }
    let len = op[1].len() - 1;
    for (i, c) in op[1].chars().enumerate() {
        *coefficients.entry(c).or_default() -= 10_i64.pow((len - i) as u32);
    }
    let mut vars: Vec<char> = terms
        .iter()
        .map(|&w| w.chars().next().unwrap())
        .chain(op[1].chars().take(1))
        .collect::<HashSet<char>>()
        .into_iter()
        .collect();
    let mut other_letters: Vec<char> = input
        .chars()
        .filter(|c| c.is_alphabetic() && !vars.contains(c))
        .collect::<HashSet<char>>()
        .into_iter()
        .collect();
    let initials_len = vars.len();
    vars.append(&mut other_letters);
    solve_inner(&coefficients, initials_len, &vars, &[], 0)
}
