#![feature(test)]
extern crate test;

use solver::*;

struct FibonacciSequence {
    current: usize,
    next: usize,
}

impl Default for FibonacciSequence {
    fn default() -> Self {
        Self {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for FibonacciSequence {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;

        Some(self.current)
    }
}

fn naive() -> usize {
    FibonacciSequence::default()
        .take_while(|x| *x < 4_000_000)
        .filter(|x| x % 2 == 0)
        .fold(0, |acc, x| acc + x)
}

struct EvenFibonacciSequence {
    current: usize,
    next: usize,
}

impl Default for EvenFibonacciSequence {
    fn default() -> Self {
        Self {
            current: 0,
            next: 2,
        }
    }
}

impl Iterator for EvenFibonacciSequence {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let new_next = 4 * self.next + self.current;
        self.current = self.next;
        self.next = new_next;

        Some(self.current)
    }
}

fn even_fibonacci() -> usize {
    EvenFibonacciSequence::default()
        .take_while(|x| *x <= 4_000_000)
        .fold(0, |acc, x| acc + x)
}

problem!(4613732, naive, even_fibonacci);