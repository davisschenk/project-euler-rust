#![feature(test)]
extern crate test;

const ANSWER: usize = 233168;

fn sum_divisible_by(n: usize, target: usize) -> usize {
    let p = target / n;
    n * (p * (p + 1)) / 2
}
fn summed_sums() -> usize {
    return sum_divisible_by(3, 999) + sum_divisible_by(5, 999) - sum_divisible_by(15, 999);
}

#[cfg(test)]
mod p001 {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_summed_sums() {
        assert_eq!(summed_sums(), ANSWER);
    }

    #[bench]
    fn bench_summed_sums(b: &mut Bencher) {
        b.iter(|| summed_sums());
    }
}
