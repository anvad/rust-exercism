#![feature(test)]
extern crate test;
use test::Bencher;

use nth_prime as np;

#[bench]
fn test_factors_include_large_prime(b: &mut Bencher) {
    b.iter(|| np::nth(10_000));
}
