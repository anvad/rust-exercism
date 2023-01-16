// use std::collections::{BTreeSet, HashSet};

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut nums = Vec::<u8>::new();
        let mut num = value;
        while num >= 10 {
            nums.push((num % 10) as u8);
            num /= 10;
        }
        nums.push(num as u8);

        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            if nums[i] != nums[j] {
                return None;
            }
            (i, j) = (i + 1, j - 1);
        }

        // let max = nums.len() - 1;
        // let max2 = max / 2;
        // for i in 0..max2 {
        //     if nums[i] != nums[max - i] {
        //         return None;
        //     }
        // }

        Some(Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    // let mut nums = HashSet::<u64>::new(); // using hashSet is slow!
    // let mut nums = BTreeSet::<u64>::new(); // BTreeSet is worse!
    let mut palindromes = Vec::<Palindrome>::new();
    for i in min..=max {
        for j in i..=max {
            let num = i * j;
            // if nums.insert(num) {
            let p = Palindrome::new(num);
            if p.is_some() {
                palindromes.push(p.unwrap())
            }
            // }
        }
    }
    if palindromes.len() > 0 {
        palindromes.sort();
        return Some((palindromes[0], *palindromes.last().unwrap()));
    }
    None
}
