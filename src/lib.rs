#![feature(test)]
#![feature(const_fn)]

use rayon::prelude::*;

fn par_square_sum(n: u128) -> u128 {
    (1..=n).into_par_iter().map(|i| i * i).sum()
}

fn square_sum(n: u128) -> u128 {
    (1..=n).map(|i| i * i).sum()
}

extern crate test;
use test::Bencher;

#[bench]
fn bench_par_square_sum(b: &mut Bencher) {
    b.iter(|| assert_eq!(par_square_sum(100_0000_u128), 333333833333500000_u128));
}

#[bench]
fn bench_square_sum(b: &mut Bencher) {
    b.iter(|| assert_eq!(square_sum(100_0000_u128), 333333833333500000_u128));
}
