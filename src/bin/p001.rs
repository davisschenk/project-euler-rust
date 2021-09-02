#![feature(test)]
extern crate test;

use solver::*;

fn sum_divisible_by(n: usize, target: usize) -> usize {
    let p = target / n;
    n * (p * (p + 1)) / 2
}

fn summed_sums() -> usize {
    sum_divisible_by(3, 999) + sum_divisible_by(5, 999) - sum_divisible_by(15, 999)
}

problem!(233168, summed_sums);
