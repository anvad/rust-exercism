#![feature(test)]
extern crate test;
use test::Bencher;

use palindrome_products::{palindrome_products, Palindrome};

#[bench]
fn test_palindrome_new_return_some(b: &mut Bencher) {
    b.iter(|| {
        for v in [1, 11, 121, 12321, 1234321, 123454321, 543212345] {
            Palindrome::new(v);
        }
    });
}

#[bench]
fn test_find_smallest_palindrome_from_four_digit_factors(b: &mut Bencher) {
    b.iter(|| palindrome_products(1000, 9999));
}
