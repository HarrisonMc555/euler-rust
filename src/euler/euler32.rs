/*
We shall say that an n-digit number is pandigital if it makes use of all the
digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through
5 pandigital.

The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing
multiplicand, multiplier, and product is 1 through 9 pandigital.

Find the sum of all products whose multiplicand/multiplier/product identity can
be written as a 1 through 9 pandigital.

HINT: Some products can be obtained in more than one way so be sure to only
include it once in your sum.
*/

use crate::digits;

use digits::Digits;
use std::collections::HashSet;

type Num = u64;

pub fn main() {
    println!("{}", solve());
}

fn solve() -> Num {
    (0..10000)
        .flat_map(move |a| (0..std::cmp::min(a, 1000)).map(move |b| (a, b)))
        .filter(|&(a, b)| is_multiplitied_pandigital(a, b))
        .map(|(a, b)| a * b)
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}

fn is_multiplitied_pandigital(a: Num, b: Num) -> bool {
    let mut found_digit = [false; 10];
    let mut count = 0;
    let all_digits = Digits::decimal(a)
        .chain(Digits::decimal(b))
        .chain(Digits::decimal(a * b));
    for digit in all_digits {
        if digit == 0 {
            return false;
        }
        match found_digit.get_mut(digit as usize) {
            Some(found) => {
                if *found {
                    return false;
                } else {
                    *found = true;
                }
            }
            None => return false,
        }
        count += 1
    }
    return count == 9;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer() {
        assert_eq!(solve(), 45228);
    }
}
