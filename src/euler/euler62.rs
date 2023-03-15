/*
The cube, 41063625 (345^3), can be permuted to produce two other cubes: 56623104 (384^3) and 66430125 (405^3). In fact,
41063625 is the smallest cube which has exactly three permutations of its digits which are also cube.

Find the smallest cube for which exactly five permutations of its digits are cube.
 */

use counter::Counter;
use std::collections::HashMap;

const BASE: usize = 10;
const NUM_PERMUTATIONS: usize = 5;

type DigitCounts = [usize; BASE];

pub fn main() {
    for num_digits in NUM_PERMUTATIONS.. {
        if let Some(permutations) = find_cube_permutations(num_digits, NUM_PERMUTATIONS) {
            // eprintln!("{:?}", permutations);
            let smallest = permutations.into_iter().min().expect("Empty permutations");
            println!("{}", smallest);
            break;
        }
    }
}

fn find_cubes(num_digits: usize) -> impl Iterator<Item = usize> {
    let min = int_pow(BASE, num_digits - 1);
    let max = int_pow(BASE, num_digits);
    (1..)
        .map(cube)
        .skip_while(move |&n| n < min)
        .take_while(move |&n| n < max)
}

fn find_cube_permutations(num_digits: usize, num_permutations: usize) -> Option<Vec<usize>> {
    if num_permutations == 0 {
        return None;
    }
    let mut digits_to_nums = HashMap::new();
    for num in find_cubes(num_digits) {
        let digits = get_digits(num);
        let nums = digits_to_nums.entry(digits).or_insert_with(Vec::new);
        nums.push(num);
        if nums.len() == num_permutations {
            return Some(nums.to_vec());
        }
    }
    None
}

fn get_digits(num: usize) -> DigitCounts {
    let digits_counter = get_digits_counter(num);
    let mut digit_counts = [0; BASE];
    for digit in 0..BASE {
        digit_counts[digit] = digits_counter[&(digit as u8)];
    }
    digit_counts
}

fn get_digits_counter(mut num: usize) -> Counter<u8> {
    let mut digits = Counter::new();
    while num > 0 {
        let digit = (num % BASE) as u8;
        digits[&digit] += 1;
        num /= BASE;
    }
    digits
}

fn cube(num: usize) -> usize {
    num * num * num
}

fn int_pow(base: usize, power: usize) -> usize {
    (0..power).fold(1, |result, _| result * base)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cube() {
        assert_eq!(8, cube(2));
        assert_eq!(27, cube(3));
        assert_eq!(1_000, cube(10));
    }

    #[test]
    fn test_find_cubes() {
        let actual = find_cubes(2).collect::<Vec<_>>();
        let expected = vec![27, 64];
        assert_eq!(expected, actual);

        let actual = find_cubes(3).collect::<Vec<_>>();
        let expected = vec![125, 216, 343, 512, 729];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_digits() {
        let digit_counts = get_digits(4827448);
        //              0  1  2  3  4  5  6  7  8  9
        let expected = [0, 0, 1, 0, 3, 0, 0, 1, 2, 0];
        assert_eq!(expected, digit_counts);
        assert_eq!(digit_counts[1], 0);
        assert_eq!(digit_counts[2], 1);
        assert_eq!(digit_counts[4], 3);
        assert_eq!(digit_counts[7], 1);
        assert_eq!(digit_counts[8], 2);
    }
}
