use std::ops::Index;

const THOUSAND_PRIMES: [u64;168] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

pub struct PrimeSieve {
    numbers: Vec<bool>,
    length: usize,
    idx: usize,
}

impl PrimeSieve {
    pub fn new(length: usize) -> Self {
        let mut s = PrimeSieve {
            numbers: vec![true; length],
            length,
            idx: 0,
        };

        s.run();

        s
    }

    pub fn run(&mut self) {
        self.numbers[0] = false;
        self.numbers[1] = false;

        for i in 2..self.length {
            if self.numbers[i] {
                for j in ((i * i)..self.length).step_by(i) {
                    self.numbers[j] = false;
                }
            }
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.numbers = vec![true ; new_size];
        self.length = new_size;
        self.run();
    }
}

impl Iterator for PrimeSieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.idx..self.length {
            if self.numbers[i] {
                self.idx = i + 1;
                return Some(i as u64);
            }
        }

        self.idx = 0;
        None
    }
}

impl Index<usize> for PrimeSieve {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.numbers[index]
    }
}

#[test]
fn test_sieve_first_thousand_iter() {
    let s = PrimeSieve::new(1000);

    for (i, s) in s.zip(THOUSAND_PRIMES.iter()) {
        assert_eq!(i, *s);
    }
}

#[test]
fn test_sieve_first_thousand_index() {
    let s = PrimeSieve::new(1000);

    for i in 0..1000 {
        assert_eq!(THOUSAND_PRIMES.contains(&i), s[i as usize]);
    }
}
