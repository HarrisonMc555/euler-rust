/*
A palindromic number reads the same both ways. The largest palindrome made from
the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
 */

fn number_is_palindrome(number: u32) -> bool {
    is_palindrome(&number.to_string())
}

fn is_palindrome(string: &str) -> bool {
    string == string.chars().rev().collect::<String>()
}
fn pairs_with_digits_desc(num_digits: u32) -> impl Iterator<Item = (u32, u32)> {
    let start = 10u32.pow(num_digits - 1);
    let end = 10u32.pow(num_digits);
    (start..end)
        .rev()
        .flat_map(move |num1| (start..=num1).rev().map(move |num2| (num1, num2)))
}

fn solve(num_digits: u32) -> u32 {
    let mut largest_palindrome = None;
    for (num1, num2) in pairs_with_digits_desc(num_digits) {
        let product = num1 * num2;
        if largest_palindrome
            .map(|largest| product < largest)
            .unwrap_or(false)
        {
            continue;
        }
        if number_is_palindrome(product) {
            largest_palindrome = Some(product);
        }
    }
    largest_palindrome.expect("no solution found")
}

const NUM_DIGITS: u32 = 3;

fn main() {
    println!("{}", solve(NUM_DIGITS));
}

#[cfg(test)]
const ANSWER: u32 = 906609;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(NUM_DIGITS));
}
