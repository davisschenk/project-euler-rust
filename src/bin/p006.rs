#![feature(test)]
extern crate test;

const ANSWER: usize = 25164150;
const LIMIT: usize = 100;

fn do_math() -> usize{
    let s = LIMIT * (LIMIT + 1) / 2;
    let s_sq = (2 * LIMIT + 1) * (LIMIT + 1) * LIMIT / 6;

    s * s - s_sq
}

#[cfg(test)]
mod p006 {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_math() {
        assert_eq!(do_math(), ANSWER);
    }

    #[bench]
    fn bench_math(b: &mut Bencher) {
        b.iter(|| do_math());
    }
}
