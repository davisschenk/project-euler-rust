#![feature(test)]
extern crate test;
use std::iter::Product;

use solver::problem;

const INPUT: &str = include_str!("../files/p008.txt");

fn solver() -> usize {    
    INPUT.chars()
         .filter(|c|c.is_digit(10)) // Remove \n characters
         .map(|c| c.to_digit(10).unwrap() as u64) // Convert chars to u64
         .collect::<Vec<u64>>() // Collect ints into a vec
         .windows(13) // Loop through windows of 13
         .map(|w| Product::product(w.iter())) // Map windows into products
         .fold(std::u64::MIN, |a, b| a.max(b)) as usize // Get max
}
problem!(23514624000usize, solver);