use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut alphametics = Alphametics::new_option(input)?;
    // alphametics.solve_inner()?;
    if alphametics.solve_inner().is_none() {
        println!("alphametics= {:?}", alphametics);
        return None;
    }
    println!("alphametics= {:?}", alphametics);
    Some(alphametics.l2n)
}

#[derive(Debug)]
struct Alphametics {
    // rhs: String,
    // lhs: Vec<String>,
    first_chars: HashSet<char>, // contains all chars that are first in their multi-char word
    columns: Vec<Vec<char>>,    // one vec per column of digits/letters
    letters: Vec<char>,
    l2n: HashMap<char, u8>,
    count: u32,         // keeps track of how many times solve_inner is called
    col_nums: Vec<u32>, // scratchpad to do sums
}

impl Alphametics {
    fn new_option(input: &str) -> Option<Self> {
        let eqn_parts: Vec<&str> = input.split("==").map(str::trim).collect();
        if eqn_parts.len() < 2 {
            // if lhs and rhs don't exist return
            return None;
        }
        let rhs = eqn_parts[1];
        let lhs: Vec<&str> = eqn_parts[0].split("+").map(str::trim).collect();
        if lhs.iter().map(|word| word.len()).max().unwrap_or(0) > rhs.len() {
            // parts (i.e. lhs) cannot be larger than the sum (i.e. rhs)
            return None;
        }
        let letters: HashSet<char> = input
            .chars()
            .filter(|&ch| !(ch == ' ' || ch == '+' || ch == '='))
            .collect();
        if letters.len() > 10 {
            return None;
        }
        let mut letters = letters.into_iter().collect::<Vec<char>>();
        letters.sort();

        let mut words = lhs;
        words.push(rhs);

        let mut first_chars = HashSet::<char>::new();
        let mut columns = vec![Vec::<char>::new(); rhs.len()];
        for word in words {
            if word.len() > 1 {
                first_chars.insert(word.chars().nth(0).unwrap());
            }
            for (i, ch) in word.chars().rev().enumerate() {
                columns[i].push(ch);
            }
        }

        Some(Self {
            // rhs: rhs.to_string(),
            // lhs: lhs.into_iter().map(String::from).collect(),
            columns,
            first_chars,
            letters,
            l2n: HashMap::<char, u8>::new(),
            count: 0,
            col_nums: vec![],
        })
    }
    /// function is recursively called, using the backtracking algo- https://algotree.org/algorithms/backtracking/
    fn solve_inner(&mut self) -> Option<()> {
        self.count += 1;
        let current = match self.letters.pop() {
            None => return Some(()), // we've run out of letters. State is NOT INvalid
            Some(ch) => ch,
        };
        let avail_nums = self.get_available_nums(current);
        for current_num in avail_nums {
            // while not out of options of possible states at this step
            self.l2n.insert(current, current_num); // update state
            if self.validate() {
                // if current state is valid
                // proceed to next step
                if self.solve_inner().is_some() {
                    return Some(()); // callee returned Some(()), so just unwind!
                }
            }
        }
        // we've explored all options at current step, so reset and backtrack
        self.l2n.remove(&current);
        self.letters.push(current);
        None
    }

    /// returns list of possible digits, after removing all currently allocated digits
    fn get_available_nums(&self, current: char) -> Vec<u8> {
        let nums = self.l2n.values().collect::<HashSet<&u8>>();
        let start = match self.first_chars.contains(&current) {
            true => 1,
            _ => 0,
        };
        (start..10).filter(|&n| !nums.contains(&n)).collect()
    }

    /// checks whether current state is invalid
    /// returns false iff we know for sure that the current state is invalid
    /// else returns true
    fn validate(&mut self) -> bool {
        // check column by column
        // if any column is missing l2n assignments, skip to next column

        for column in self.columns.iter() {
            let mut fully_assigned = true;
            for ch in column.iter() {
                if !self.l2n.contains_key(ch) {
                    // this column is not fully assigned, so skip to next column
                    // println!("column {:?} is not fully assigned. {:?}", column, self.l2n);

                    // this column is not fully assigned, so skip all remaining checks
                    fully_assigned = false;
                    self.col_nums.clear();
                    return true;
                } else {
                    // note: last item pushed is the result number
                    self.col_nums
                        .push(self.l2n.get(ch).copied().unwrap() as u32);
                }
            }
            if fully_assigned {
                let result = self.col_nums.last().unwrap_or(&0);
                let sum = self.col_nums.iter().fold(0u32, |sum, n| sum + *n) - *result;
                if sum % 10 != *result {
                    // we know for sure that is assignment is invalid
                    // println!(
                    //     "column: {:?}, col_nums: {:?}, l2n: {:?}, sum: {sum}, result: {}",
                    //     column, self.col_nums, self.l2n, result
                    // );
                    self.col_nums.clear();
                    return false;
                } else {
                    // this column checks out, let's carry over and check next column
                    self.col_nums.clear();
                    self.col_nums.push(sum / 10);
                }
            }
        }
        // we've checked all columns.
        //  either they are not fully assigned, or their sums have checked out
        self.col_nums.clear();
        true
    }
}
