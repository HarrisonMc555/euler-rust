/*
The sum of the squares of the first ten natural numbers is,

1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,

(1 + 2 + ... + 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural
numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred
natural numbers and the square of the sum.
*/

fn sum_of_squares(limit: u64) -> u64 {
    let n = limit;
    (n * (n + 1) * (2 * n + 1)) / 6
}

fn square_of_sum(limit: u64) -> u64 {
    let n = limit;
    let sum = (n * (n + 1)) / 2;
    sum * sum
}

fn solve(limit: u64) -> u64 {
    square_of_sum(limit) - sum_of_squares(limit)
}

const LIMIT: u64 = 100;

fn main() {
    println!("{}", solve(LIMIT));
}

#[cfg(test)]
const ANSWER: u64 = 25164150;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(LIMIT));
}
