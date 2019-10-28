/*
A permutation is an ordered arrangement of objects. For example, 3124 is one
possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are
listed numerically or alphabetically, we call it lexicographic order. The
lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5,
6, 7, 8 and 9?
*/

fn solve(nth: usize, min_num: u8, max_num: u8) -> usize {
    let index = nth - 1;
    let options = (min_num..=max_num).collect();
    let digits = find_permutation(index, options);
    from_digits(&digits)
}

fn find_permutation<T>(mut index: usize, mut options: Vec<T>) -> Vec<T>
{
    let mut result = Vec::new();
    for num_left in (0..options.len()).rev() {
        let divisor = factorial(num_left);
        let options_index = index / divisor;
        result.push(options.remove(options_index));
        index %= divisor;
    }
    result
}

fn factorial(num: usize) -> usize {
    (1..=num).product()
}

fn from_digits(digits: &[u8]) -> usize {
    digits.iter().fold(0, |acc, digit| acc * BASE + (*digit as usize))
}

const BASE: usize = 10;

const NTH: usize = 1_000_000;
const MIN_NUM: u8 = 0;
const MAX_NUM: u8 = 9;

fn main() {
    println!("{}", solve(NTH, MIN_NUM, MAX_NUM));
}

#[cfg(test)]
const ANSWER: usize = 2_783_915_460;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(NTH, MIN_NUM, MAX_NUM));
}
