/*
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,

and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 =
27.

Find the sum of the digits in the number 100!
*/

use num_bigint::BigUint;
use num_traits::One;

fn factorial(num: u32) -> BigUint {
    num_iter::range_inclusive(BigUint::one(), num.into()).product()
}

fn solve(num: u32) -> u64 {
    factorial(num)
        .to_string()
        .chars()
        .flat_map(|c| c.to_digit(BASE))
        .map(u64::from)
        .sum()
}

const INPUT: u32 = 100;
const BASE: u32 = 10;

fn main() {
    println!("{}", solve(INPUT));
}

#[cfg(test)]
const ANSWER: u64 = 648;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(INPUT));
}

