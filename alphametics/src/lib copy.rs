use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // each letter must have a unique number
    // identify all the letters
    //  for each letter, put an initial list of possible numbers
    //  some will be allowed to use 0-9
    //  others will be restriced to 1-9
    //  then, as we iterate thru the letters, the next letter will have 1 less number
    //  in its list of possible numbers
    // even with brute force, how to iterate thru a variable number of nested loops? recursion?
    let l2n = HashMap::<char, u8>::new();
    let n2l = HashMap::<u8, char>::new();
    let eqn_parts: Vec<&str> = input.split("==").map(str::trim).collect();
    if eqn_parts.len() < 2 {
        return None;
    }
    let rhs = eqn_parts[1];
    let lhs_parts: Vec<&str> = eqn_parts[0].split("+").map(str::trim).collect();
    if lhs_parts.iter().map(|word| word.len()).max().unwrap_or(0) > rhs.len() {
        return None;
    }
    let letters: HashSet<char> = input
        .chars()
        .filter(|&ch| !(ch == ' ' || ch == '+' || ch == '='))
        .collect();
    let mut letters = letters.into_iter().collect::<Vec<char>>();
    letters.sort();
    // println!("unique letters= {:?}\n{:?}\n{:?}", letters, rhs, lhs_parts);
    // permutations and combinations
    // first select the needed number of digits, then permute among them
    // then assign the digits to the letters?

    // how to select r items from n? nCr is the number of ways of doing this.
    //  nCr = n!/(n-r)!/r!
    //  nPr = n!/(n-r)!
    // create a binary number of n digits with r of them set to 1.
    // then, each number that has exactly r 1s is a valid selection.
    // let's say we are selecting 4 1s out of 10
    //  start with smallest such valid number: 1111
    //  then, add 1, we get 10000, not ok, then + 1 etc?
    // next largest valid number is:  10111 (diff = 1000)
    // next largest valid number is:  11011 (diff = 100)
    // next largest valid number is:  11101 (diff = 10)
    // next largest valid number is:  11110 (diff = 1)
    // next largest valid number is:  11110 (diff = 1)
    // next largest valid number is: 101110 (diff = 1000)
    // hey, i am seeing a pattern here!

    // once the numbers are selected, how to enumerate all possible permutations?
    // say we have 4 digits and 4 letters. we should have 4*3*2*1 = 24 ways
    // can we make a 4x4 matrix?
    //   2 3 5 8
    // A 1 0 0 0
    // E 0 1 0 0
    // R 0 0 1 0
    // T 0 0 0 1
    // searching on the net, found https://en.wikipedia.org/wiki/Heap%27s_algorithm

    return Some(l2n);
}

// see https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn factorial(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}
fn generate<T: Copy>(k: usize, arr: &mut Vec<T>) {
    if k == 1 {
        return;
    }
    // Generate permutations with k-th unaltered
    // Initially k = length(arr)
    generate(k - 1, arr);

    // Generate permutations for k-th swapped with each k-1 initial
    for i in 0..(k - 1) {
        // Swap choice dependent on parity of k (even or odd)
        if k % 2 == 0 {
            // swap(A[i], A[k-1]) // zero-indexed, the k-th is at k-1
            (arr[i], arr[k - 1]) = (arr[k - 1], arr[i]);
        } else {
            // swap(A[0], A[k-1])
            (arr[0], arr[k - 1]) = (arr[k - 1], arr[0]);
        }
        generate(k - 1, arr);
    }
}
