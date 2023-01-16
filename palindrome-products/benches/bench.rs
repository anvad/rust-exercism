#![feature(test)]
extern crate test;
use test::Bencher;

use palindrome_products::Palindrome;

#[bench]
fn test_palindrome_new_return_some(b: &mut Bencher) {
    b.iter(|| {
        for v in [1, 11, 121, 12321, 1234321, 123454321, 543212345] {
            Palindrome::new(v);
        }
    });
}
