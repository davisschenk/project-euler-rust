#![feature(test)]
extern crate test;
use solver::problem;
const LIMIT: usize = 100;

fn do_math() -> usize {
    let s = LIMIT * (LIMIT + 1) / 2;
    let s_sq = (2 * LIMIT + 1) * (LIMIT + 1) * LIMIT / 6;

    s * s - s_sq
}

problem!(25164150, do_math);