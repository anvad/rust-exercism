use std::collections::{HashMap, HashSet};

// core logic remains same. More optimization in the `validate` function
//  now also uses `cols_map` and `solutions_char` to speed up summing
//  this had big impact on the `test_puzzle_with_ten_letters_and_199_addends` test
//   as that has many repetitive letters, so instead of looping thru each char in a column,
//   we just loop thru each unique char and multiply with how often it is present in that column,
//  that shaved time for the `test_puzzle_with_ten_letters` from 2.7ms to 2.2ms
//  and the all test cases runtime went from approx. 10 secs to 2.5 secs

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut alphametics = Alphametics::new_option(input)?;
    alphametics.solve_inner()?;
    Some(alphametics.get_char_u8_map())
}

#[derive(Debug)]
struct Alphametics {
    first_chars: HashSet<char>, // contains all chars that are first in their multi-char word
    solution_chars: Vec<char>,
    cols_map: Vec<HashMap<char, u32>>, // one vec per column of digits/chars
    col_char_count: Vec<usize>,        // cumulative number of unique chars encountered by this col
    chars: Vec<char>,                  // list of all unique chars in the puzzle, sorted by column
    c2n: HashMap<char, u32>,           // char-to-number mapping. This is the solution candidate
    avail_nums: HashSet<u32>,          // digits that are available to be assigned
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

        // we want to arrange the letters in the `chars` array, in such a way,
        //  as to fully assign column-0, then fully assign column-1, etc.
        //  that way, we can avoid a lot of validation checks
        let mut chars = Vec::<char>::new();
        let mut chars_seen_so_far = HashSet::<char>::new();
        let mut col_char_count = Vec::<usize>::new();
        for column in columns.iter() {
            for ch in column.iter() {
                if !chars_seen_so_far.contains(ch) {
                    chars.push(*ch);
                    chars_seen_so_far.insert(*ch);
                }
            }
            col_char_count.push(chars_seen_so_far.len());
        }

        let cols_map = columns
            .iter()
            .map(|column| {
                column
                    .iter()
                    .fold(HashMap::<char, u32>::new(), |mut col_map, ch| {
                        *col_map.entry(*ch).or_insert(0) += 1;
                        col_map
                    })
            })
            .collect();

        // this revers() ensures pop letters from the units column first, then tens, etc.
        //  so, we can skip looking at later columns whose letters have not yet been assigned
        chars.reverse();
        println!("chars after: {:?} {:?}", chars, col_char_count);

        Some(Self {
            first_chars,
            solution_chars: rhs.chars().rev().collect(),
            cols_map,
            chars,
            col_char_count,
            c2n: HashMap::<char, u32>::new(),
            avail_nums: (0..10).into_iter().collect(), // we'll add 0 for non-first-chars
        })
    }

    /// function is recursively called, using the backtracking algo- https://algotree.org/algorithms/backtracking/
    fn solve_inner(&mut self) -> Option<()> {
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
    fn get_available_nums(&self, current: char) -> Vec<u32> {
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
        let mut sum = 0u32;
        for col_index in 0..self.solution_chars.len() {
            let col_char_count = self.col_char_count[col_index];
            if col_char_count > self.c2n.len() {
                // this column is not fully assigned, so skip all remaining checks
                return true;
            }
            for (ch, &count) in self.cols_map[col_index].iter() {
                // note: count includes the expected sum
                //  so, later, we'll have to subtract it from sum
                let mapped_number = *self.c2n.get(ch).unwrap();
                sum += mapped_number * count;
            }

            // we are here, means column is fully assigned
            //  so, now validate the sum = result
            let expected_sum = *self.c2n.get(&self.solution_chars[col_index]).unwrap();
            sum -= expected_sum;
            if sum % 10 != expected_sum {
                // we know for sure that is assignment is invalid
                return false;
            } else {
                // this column checks out, let's carry over and check next column
                sum /= 10;
            }
        }
        // we've checked all columns.
        //  either they are not fully assigned, or their sums have checked out
        true
    }

    fn get_char_u8_map(&mut self) -> HashMap<char, u8> {
        self.c2n
            .iter()
            .fold(HashMap::<char, u8>::new(), |mut c2n, (k, v)| {
                c2n.insert(*k, *v as u8);
                c2n
            })
    }
}
