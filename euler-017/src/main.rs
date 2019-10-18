/*
If the numbers 1 to 5 are written out in words: one, two, three, four, five,
then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in
words, how many letters would be used?


NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and
forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20
letters. The use of "and" when writing out numbers is in compliance with British
usage.
*/

mod say;

fn count_letters(word: &str) -> usize {
    word.chars().filter(|c| c.is_alphabetic()).count()
}

fn solve(begin: u64, end_inclusive: u64) -> usize {
    (begin..=end_inclusive)
        .map(|num| count_letters(&say::encode(num)))
        .sum()
}

const BEGIN: u64 = 1;
const END_INCLUSIVE: u64 = 1_000;

fn main() {
    println!("{}", solve(BEGIN, END_INCLUSIVE));
}

#[cfg(test)]
const ANSWER: usize = 21_124;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(BEGIN, END_INCLUSIVE));
}
