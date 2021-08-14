#![feature(test)]
extern crate test;
use solver::problem;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
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

problem!(232792560, use_lcm);
