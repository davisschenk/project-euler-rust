#![feature(test)]
extern crate test;

use num::Integer;
use solver::problem;

const N: f64 = 1000.0;

fn brute_force() -> usize {
    let a_max = ((N - 3.0) / 2.0).round() as usize;
    for a in 3..a_max {
        let b_max = ((N - 1.0 - a as f64) / 2.0).round() as usize;
        for b in a + 1..b_max {
            let c = N as usize - a - b;

            if c * c == a * a + b * b {
                return (a * b * c) as usize;
            }
        }
    }

    unreachable!()
}

fn parametrisation() -> usize {
    let s2 = N / 2.0;
    let mlimit = s2.sqrt().ceil() - 1.0;

    for m in 2..mlimit as usize {
        if s2 as usize % m == 0 {
            let mut sm = s2 as usize / m;
            while sm % 2 == 0 {
                sm /= 2;
            }

            let mut k = match m % 2 {
                1 => m + 2,
                _ => m + 1,
            };

            while k < 2 * m && k <= sm {
                if sm % k == 0 && k.gcd(&m) == 1 {
                    let d = s2 / (k * m) as f64;
                    let n = (k - m) as f64;
                    let a = d * (m as f64 * m as f64 - n * n);
                    let b = 2.0 * d * m as f64 * n;
                    let c = d * (m as f64 * m as f64 + n * n);

                    return (a * b * c) as usize;
                }

                k += 2;
            }
        }
    }

    unreachable!()
}

problem!(31875000, brute_force, parametrisation);
