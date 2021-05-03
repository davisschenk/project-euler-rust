#![feature(test)]
extern crate test;

const ANSWER: usize = 232792560;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a
    }

    gcd(b, a % b)
}

fn lcm(mut multiples: impl Iterator<Item = usize>) -> usize {
    let mut ans = multiples.next().unwrap();
    
    for i in multiples {
        ans = i * ans / gcd(i, ans)
    }

    return ans;
}

fn use_lcm() -> usize {
    lcm(1..20)
}

#[cfg(test)]
mod p005 {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_lcm() {
        assert_eq!(use_lcm(), ANSWER);
    }

    #[bench]
    fn bench_lcm(b: &mut Bencher) {
        b.iter(|| use_lcm());
    }
}
