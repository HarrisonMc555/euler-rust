/*
2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
 */

use num_bigint::BigUint;
use num_traits::pow::Pow;

fn solve(base: u32, exponent: u32) -> u32 {
    let number = BigUint::from(base).pow(exponent);
    let digits = number.to_radix_be(NUMBER_SYSTEM_BASE);
    digits.iter().cloned().map(u32::from).sum()
}

const BASE: u32 = 2;
const EXPONENT: u32 = 1_000;

const NUMBER_SYSTEM_BASE: u32 = 10;

fn main() {
    println!("{}", solve(BASE, EXPONENT));
}

const ANSWER: u32 = 1366;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(BASE, EXPONENT));
}
