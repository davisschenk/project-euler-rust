#![feature(test)]
extern crate test;
use solver::problem;

use prime::PrimeSieve;

fn solver() -> usize {
    let n: f64 = 10001.0;
    let limit = (n * n.ln() + n * (n - 0.9385).ln().ln()).round();
    let mut sieve = PrimeSieve::new(limit as usize);

    return sieve.nth(10000).unwrap() as usize;
}

problem!(104743, solver);