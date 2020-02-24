/*
Surprisingly there are only three numbers that can be written as the sum of
fourth powers of their digits:

1634 = 1^4 + 6^4 + 3^4 + 4^4
8208 = 8^4 + 2^4 + 0^4 + 8^4
9474 = 9^4 + 4^4 + 7^4 + 4^4
As 1 = 1^4 is not a sum it is not included.

The sum of these numbers is 1634 + 8208 + 9474 = 19316.

Find the sum of all the numbers that can be written as the sum of fifth powers
of their digits.
*/

mod digits;

use digits::Digits;

fn main() {
    println!("{}", solve(5));
}

fn solve(power: u32) -> u64 {
    (2..1_000_000)
        .filter(|&n| is_sum_of_powered_digits(n, power))
        .sum()
}

fn is_sum_of_powered_digits(num: u64, power: u32) -> bool {
    let sum: u64 = Digits::decimal(num)
        .map(|digit| u64::from(digit).pow(power))
        .sum();
    sum == num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve(4), 19_316);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(5), 443_839);
    }
}
