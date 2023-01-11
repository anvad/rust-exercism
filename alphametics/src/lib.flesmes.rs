// flesmes's solution

// Fast Alphametics solver. It takes 1.50s to run the test suite
// Starts matching the most significant digits, using carry digits as needed,
// and ends with the least significant ones
use std::cmp;
use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub struct Alphametics {
    pub addends: Vec<Vec<char>>,
    pub addition: Vec<char>,
}
impl Alphametics {
    /// Addend letter: Addend at index `term` and level `level`
    pub fn addend_letter(
        &self,
        char_digit_map: HashMap<char, u8>,
        level: usize,
        mut term: usize,
    ) -> Option<HashMap<char, u8>> {
        // Work with addend `term` if it has digits at level `level`,
        // otherwise go to the next addend that has digits at this level
        while term < self.addends.len() && self.addends[term].len() < level + 1 {
            term += 1
        }
        if term == self.addends.len() {
            // No more terms, match addition digit
            return self.addition_letter(char_digit_map, level);
        }
        if char_digit_map.contains_key(&self.addends[term][level]) {
            // Letter already in char_digit_map
            // Process next digit and return result if it is a solution
            let result = self.finish_addend_letter(char_digit_map, level, term);
            if result.is_some() {
                return result;
            }
        } else {
            // Letter not in char_digit_map
            let used_digits: Vec<u8> = char_digit_map.values().copied().collect();
            for digit in 0..=9 {
                if
                // digit not already used and
                ! used_digits.contains(&digit) &&
                   // not leading zero
                   ( digit != 0 || level != self.addends[term].len() - 1 )
                {
                    // add digit to new char_digit_map
                    let mut char_digit_map = char_digit_map.clone();
                    char_digit_map.insert(self.addends[term][level], digit);

                    // Process next digit and return result if it's solution
                    let result = self.finish_addend_letter(char_digit_map, level, term);
                    if result.is_some() {
                        return result;
                    }
                }
            }
        }
        None
    }
    // Continue with the next significant addend or with the addition term if no more addends
    pub fn finish_addend_letter(
        &self,
        char_digit_map: HashMap<char, u8>,
        level: usize,
        mut term: usize,
    ) -> Option<HashMap<char, u8>> {
        //  Go to the next addend that has digits at this level
        term += 1;
        while term < self.addends.len() && self.addends[term].len() < level + 1 {
            term += 1
        }
        if term == self.addends.len() {
            // process addition digit at this level
            self.addition_letter(char_digit_map, level)
        } else {
            // process next addend digit
            self.addend_letter(char_digit_map, level, term)
        }
    }
    /// Addition term letter at level `level`
    pub fn addition_letter(
        &self,
        char_digit_map: HashMap<char, u8>,
        level: usize,
    ) -> Option<HashMap<char, u8>> {
        if char_digit_map.contains_key(&self.addition[level]) {
            self.addition_used_letter(char_digit_map, level)
        } else {
            self.addition_not_used_letter(char_digit_map, level)
        }
    }
    // Addition term letter at level `level`. Letter already in char_digit_map
    fn addition_used_letter(
        &self,
        char_digit_map: HashMap<char, u8>,
        level: usize,
    ) -> Option<HashMap<char, u8>> {
        let sum_addends = self.addend_sum(level, &char_digit_map);
        // carry is an unknown, bound by 0 and `max_carry`
        let max_carry = self.maximum_carry(level);
        let addition_substitution: u64 = substitute(&self.addition[level..], &char_digit_map);
        for carry in 0..=max_carry {
            let carried_sum = sum_addends + carry;
            if addition_substitution == carried_sum {
                if level == 0 {
                    return Some(char_digit_map);
                } else {
                    let result = self.addend_letter(char_digit_map.clone(), level - 1, 0);
                    if result.is_some() {
                        return result;
                    }
                }
            }
        }
        None
    }
    // Addition term letter at level `level`. Letter not in char_digit_map
    fn addition_not_used_letter(
        &self,
        mut char_digit_map: HashMap<char, u8>,
        level: usize,
    ) -> Option<HashMap<char, u8>> {
        let sum_addends = self.addend_sum(level, &char_digit_map);
        // carry is an unknown, bound by 0 and `max_carry`
        let max_carry = self.maximum_carry(level);
        let values: Vec<u8> = char_digit_map.values().copied().collect();
        for carry in 0..=max_carry {
            let carried_sum = sum_addends + carry;
            // Check match at upper levels
            if level < self.addition.len() - 1 {
                let addition_substitution: u64 =
                    substitute(&self.addition[level + 1..], &char_digit_map);
                if carried_sum / 10 != addition_substitution {
                    // no match
                    continue;
                }
            }
            let addition_digit = (carried_sum % 10) as u8;
            // Check digit not used
            if values.contains(&addition_digit) {
                continue;
            }
            // Check not leading zero
            if addition_digit == 0 && level == self.addition.len() - 1 {
                continue;
            }
            // Add letter to char_digit_map
            char_digit_map.insert(self.addition[level], addition_digit);
            // Next recursion or finish
            if level == 0 {
                return Some(char_digit_map);
            } else {
                let result = self.addend_letter(char_digit_map.clone(), level - 1, 0);
                if result.is_some() {
                    return result;
                }
            }
        }
        None
    }
    // Sum of addends for letters above level `level`
    fn addend_sum(&self, level: usize, char_digit_map: &HashMap<char, u8>) -> u64 {
        self.addends
            .iter()
            .filter(|addend| addend.len() > level)
            .map(|addend| substitute(&addend[level..], &char_digit_map))
            .sum()
    }
    // Maximum carry at level `level`
    fn maximum_carry(&self, level: usize) -> u64 {
        self.addends
            .iter()
            .map(|addend| 10_u64.pow(cmp::min(addend.len(), level) as u32) - 1)
            .sum::<u64>()
            / 10_u64.pow(level as u32)
    }
}
// Solve alphametic
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let pattern = parse(input);
    if pattern.addends.iter().map(Vec::len).max().unwrap() > pattern.addition.len() {
        return None;
    }
    pattern.addend_letter(HashMap::new(), pattern.addition.len() - 1, 0)
}
// Parse string and get terms as vectors of chars
pub fn parse(input: &str) -> Alphametics {
    let mut input_iter = input.split("==");
    let mut addends = vec![];
    let terms = input_iter.next().unwrap();
    for term in terms.split("+") {
        addends.push(term.trim().chars().rev().collect());
    }
    let addition = input_iter.next().unwrap().trim().chars().rev().collect();
    Alphametics { addends, addition }
}
// Vector of digits to integer
pub fn to_number(digits: &[u8], base: u64) -> u64 {
    digits
        .iter()
        .copied()
        .fold(0, |acc, d| acc * base + (d as u64))
}
// Vector of chars to integer
pub fn substitute(pattern: &[char], digit_map: &HashMap<char, u8>) -> u64 {
    const BASE: u64 = 10;
    let mut substitution: Vec<u8> = pattern.iter().map(|c| digit_map[c]).collect();
    substitution.reverse();
    to_number(&substitution, BASE)
}
