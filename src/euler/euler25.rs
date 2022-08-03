/*
The Fibonacci sequence is defined by the recurrence relation:

F_n = F_n−1 + F_n−2, where F_1 = 1 and F_2 = 1.

Hence the first 12 terms will be:

F_1 = 1
F_2 = 1
F_3 = 2
F_4 = 3
F_5 = 5
F_6 = 8
F_7 = 13
F_8 = 21
F_9 = 34
F_10 = 55
F_11 = 89
F_12 = 144

The 12th term, F12, is the first term to contain three digits.

What is the index of the first term in the Fibonacci sequence to contain 1000
digits?
*/

use num_bigint::BigUint;
use num_traits::identities::One;

fn solve(num_digits: usize) -> usize {
    let index = Fibonacci::new()
        .take_while(|num| get_num_digits(num) < num_digits)
        .count();
    index + 1
}

fn get_num_digits(num: &BigUint) -> usize {
    num.to_str_radix(BASE).chars().count()
}

struct Fibonacci {
    prev: BigUint,
    next: BigUint,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci {
            prev: BigUint::one(),
            next: BigUint::one(),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let old_prev = self.prev.clone();
        std::mem::swap(&mut self.prev, &mut self.next);
        self.next += &self.prev;
        Some(old_prev)
    }
}

const BASE: u32 = 10;
const NUM_DIGITS: usize = 1_000;

pub fn main() {
    println!("{}", solve(NUM_DIGITS));
}

#[cfg(test)]
const ANSWER: usize = 4_782;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(NUM_DIGITS));
}
