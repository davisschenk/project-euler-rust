#![feature(test)]
extern crate test;

use std::cmp::max;
use solver::problem;

fn reverse_digits(n: usize, radix: usize) -> usize {
    let mut nc = n;
    let mut reverse = 0;

    while nc > 0 {
        reverse *= radix;
        reverse += nc % radix;
        nc /= radix;
    }

    reverse
}

fn is_palindrome(number: usize) -> bool {
    number == reverse_digits(number, 10)
}

fn improved_search() -> usize {
    let mut best = 0;
    let mut step = 11;
    let mut j_max = 999;

    for i in (99..j_max + 1).step_by(step).rev() {
        println!("{}", i);
        if i % 11 == 0 {
            j_max = 999;
            step = 1;
        } else {
            j_max = 990;
            step = 11;
        }

        for j in (99..j_max + 1).step_by(step).rev() {
            if i * j <= best {
                break;
            }

            if is_palindrome(j * i) {
                best = max(best, j * i);
            }
        }
    }

    best
}

problem!(906609, improved_search);
