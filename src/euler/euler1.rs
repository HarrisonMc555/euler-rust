/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get
3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

fn divisble_by(num: u32, divisor: u32) -> bool {
    num % divisor == 0
}

fn divisble_by_any(num: u32, divisors: &[u32]) -> bool {
    divisors.iter().any(|divisor| divisble_by(num, *divisor))
}

fn solve(divisors: &[u32], limit: u32) -> u64 {
    let multiples = (1..limit).filter(|num| divisble_by_any(*num, divisors));
    // To make this a more general solution, we're summing into a u64, since the
    // sum of multiple u32s could overflow a single u32. However, that means we
    // can't use the built-in sum method.
    multiples.fold(0, |sum, num| sum + u64::from(num))
}

const LIMIT: u32 = 1_000;
const DIVISORS: [u32; 2] = [3, 5];

pub fn main() {
    println!("{}", solve(&DIVISORS, LIMIT));
}

#[cfg(test)]
const ANSWER: u64 = 233168;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(&DIVISORS, LIMIT));
}
