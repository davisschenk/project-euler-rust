#![feature(test)]
extern crate test;

use solver::problem;
const NUMBER: usize = 600851475143;

fn check_all() -> usize {
    let mut factor = 2;
    let mut last = 1;
    let mut n = NUMBER;

    while n > 1 {
        if n % factor == 0 {
            last = factor;
            n /= factor;

            while n % factor == 0 {
                n /= factor;
            }
        }

        factor += 1;
    }

    last
}

fn squared() -> usize {
    let mut n = NUMBER;
    let mut last_factor;

    if n % 2 == 0 {
        last_factor = 2;
        n /= 2;

        while n % 2 == 0 {
            n /= 2;
        }
    } else {
        last_factor = 1;
    }

    let mut factor = 3;
    let mut max_factor: f64 = (n as f64).sqrt();

    while n > 1 && factor as f64 <= max_factor {
        if n % factor == 0 {
            n /= factor;
            last_factor = factor;
            while n % factor == 0 {
                n /= factor;
            }
            max_factor = (n as f64).sqrt();
        }
        factor += 1;
    }

    if n == 1 {
        return last_factor;
    }
    n
}

problem!(6857, squared, check_all);
