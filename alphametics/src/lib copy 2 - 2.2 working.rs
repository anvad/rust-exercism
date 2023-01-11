use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut alphametics = Alphametics::new_option(input)?;
    // alphametics.solve_inner()?;
    if alphametics.solve_inner().is_none() {
        println!(
            "alphametics= {}, {}",
            alphametics.count_solver, alphametics.count_val
        );
        return None;
    }
    println!(
        "alphametics= {}, {}",
        alphametics.count_solver, alphametics.count_val
    );
    Some(alphametics.c2n)
}

#[derive(Debug)]
struct Alphametics {
    first_chars: HashSet<char>, // contains all chars that are first in their multi-char word
    columns: Vec<Vec<char>>,    // one vec per column of digits/chars
    // solution_chars: Vec<char>,  // chars from the solution (rhs of equation)
    // col_chars: Vec<Vec<char>>,  // list of all unique chars, by column in which they first appear
    chars: Vec<char>,        // list of all unique chars in the puzzle
    c2n: HashMap<char, u8>,  // solution where we've assigned a number to each unique char
    count_solver: u32,       // keeps track of how many times solve_inner is called
    count_val: u32,          // keeps track of how many times validate is called
    col_nums: Vec<u32>,      // scratchpad to do sums
    avail_nums: HashSet<u8>, // digits that are available to be assigned
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

        let mut words = lhs;
        words.push(rhs);

        let mut first_chars = HashSet::<char>::new();
        let mut columns = vec![Vec::<char>::new(); rhs.len()];
        for word in words.iter() {
            if word.len() > 1 {
                first_chars.insert(word.chars().nth(0).unwrap());
            }
            for (i, ch) in word.chars().rev().enumerate() {
                columns[i].push(ch);
            }
        }
        // let lhs_len = words.len() - 1;
        // for word in words[0..lhs_len].iter() {
        //     for (i, ch) in word.chars().rev().enumerate() {
        //         columns[i].push(ch);
        //     }
        // }

        // we want to arrange the letters in the `chars` array, in such a way,
        //  as to fully assign column-0, then fully assign column-1, etc.
        //  that way, we can avoid a lot of validate checks
        let mut chars = Vec::<char>::new();
        let mut chars_seen_so_far = HashSet::<char>::new();
        for column in columns.iter() {
            for ch in column.iter() {
                if !chars_seen_so_far.contains(ch) {
                    chars.push(*ch);
                    chars_seen_so_far.insert(*ch);
                }
            }
        }

        // let solution_chars = rhs.chars().collect();

        // this revers() ensures pop letters from the units column first, then tens, etc.
        //  so, we can skip looking at later columns whose letters have not yet been assigned
        chars.reverse();
        // println!("chars after: {:?}", chars);

        Some(Self {
            first_chars,
            columns,
            // solution_chars,
            chars,
            c2n: HashMap::<char, u8>::new(),
            count_solver: 0,
            count_val: 0,
            col_nums: vec![],
            avail_nums: (0..10).into_iter().collect(),
        })
    }

    /// function is recursively called, using the backtracking algo- https://algotree.org/algorithms/backtracking/
    fn solve_inner(&mut self) -> Option<()> {
        self.count_solver += 1;
        let current = match self.chars.pop() {
            None => return Some(()), // we've run out of letters. State is NOT INvalid
            Some(ch) => ch,
        };
        let avail_nums = self.get_available_nums(current);

        for current_num in avail_nums {
            // 1. while not out of options of possible states at this step
            //    update state and check if this is valid state
            self.c2n.insert(current, current_num);
            if self.validate() {
                // 2. if current state is valid
                //    proceed to next (child) step but
                //    first, update avail_nums before recursive call
                self.avail_nums.remove(&current_num);
                if self.solve_inner().is_some() {
                    return Some(()); // callee returned Some(()), so just unwind!
                }
                // 3. the child step failed, so move to next option
                //    but revert state before that
                self.avail_nums.insert(current_num);
            }
        }
        // we've explored all options at current step, so reset and backtrack
        self.c2n.remove(&current);
        self.chars.push(current);
        None
    }

    /// returns list of possible digits, after removing all currently allocated digits
    fn get_available_nums(&self, current: char) -> Vec<u8> {
        let start = match self.first_chars.contains(&current) {
            true => 1,
            _ => 0,
        };
        (start..10)
            .filter(|n| self.avail_nums.contains(n))
            .collect()
    }

    /// checks whether current state is invalid
    /// returns false iff we know for sure that the current state is invalid
    /// else returns true
    fn validate(&mut self) -> bool {
        // check column by column, starting from units column
        // if any column is missing c2n assignments, skip further checking because
        //  we can't check subsequent columns without knowing whether there is a carry
        self.count_val += 1;
        for column in self.columns.iter() {
            for ch in column.iter() {
                if !self.c2n.contains_key(ch) {
                    // this column is not fully assigned, so skip all remaining checks
                    self.col_nums.clear();
                    return true;
                } else {
                    // note: last item pushed is the result number
                    self.col_nums.push(*self.c2n.get(ch).unwrap() as u32);
                }
            }

            // we are here, means column is fully assigned
            let result = *self.col_nums.last().unwrap_or(&0);
            // let last = self.col_nums.len() - 1;
            // let result = self.col_nums[last];
            // let result = self.solution_chars[i];
            let sum = self.col_nums.iter().fold(0u32, |sum, n| sum + *n) - result;
            if sum % 10 != result {
                // we know for sure that is assignment is invalid
                // println!(
                //     "column: {:?}, col_nums: {:?}, c2n: {:?}, sum: {sum}, result: {}",
                //     column, self.col_nums, self.c2n, result
                // );
                self.col_nums.clear();
                return false;
            } else {
                // this column checks out, let's carry over and check next column
                self.col_nums.clear();
                self.col_nums.push(sum / 10);
            }
        }
        // we've checked all columns.
        //  either they are not fully assigned, or their sums have checked out
        self.col_nums.clear();
        true
    }
}
