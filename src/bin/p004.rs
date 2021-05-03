#![feature(test)]
extern crate test;

use std::cmp::max;

const ANSWER: usize = 906609;

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

#[cfg(test)]
mod p004 {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_improved_search() {
        assert_eq!(improved_search(), ANSWER);
    }

    #[bench]
    fn bench_improved_search(b: &mut Bencher) {
        b.iter(|| improved_search());
    }

    #[test]
    #[ignore]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(343), true);
        assert_eq!(is_palindrome(123), false);
        assert_eq!(is_palindrome(609906), true);
        assert_eq!(is_palindrome(29), false);
    }

    #[bench]
    #[ignore]
    fn bench_is_palindrome(b: &mut Bencher) {
        b.iter(|| is_palindrome(404));
    }
}
