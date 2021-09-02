#![feature(test)]
extern crate test;
use solver::problem;

use prime::PrimeSieve;

const N: f64 = 10001.0;

fn solver() -> usize {
    let limit = (N * N.ln() + N * (N - 0.9385).ln().ln()).round();
    let mut sieve = PrimeSieve::new(limit as usize);

    return sieve.nth(10000).unwrap() as usize;
}

problem!(104743, solver);
