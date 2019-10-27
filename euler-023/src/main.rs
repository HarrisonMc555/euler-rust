/*
A perfect number is a number for which the sum of its proper divisors is exactly
equal to the number. For example, the sum of the proper divisors of 28 would be
1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.

A number n is called deficient if the sum of its proper divisors is less than n
and it is called abundant if this sum exceeds n.

As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest
number that can be written as the sum of two abundant numbers is 24. By
mathematical analysis, it can be shown that all integers greater than 28123 can
be written as the sum of two abundant numbers. However, this upper limit cannot
be reduced any further by analysis even though it is known that the greatest
number that cannot be expressed as the sum of two abundant numbers is less than
this limit.

Find the sum of all the positive integers which cannot be written as the sum of
two abundant numbers.
*/

use multiset::HashMultiSet;
use std::collections::HashSet;
use std::iter;

fn get_divisors(number: usize, primes: &[usize]) -> HashSet<usize> {
    let factor_multiset = get_factors(number, primes);
    let factors_pairs = iter_counts(&factor_multiset)
        .map(|(&factor, count)| (factor, count))
        .collect::<Vec<(usize, usize)>>();
    let mut divisors = factors_to_divisors(&factors_pairs);
    divisors.remove(&number);
    divisors
}

fn factors_to_divisors(factors: &[(usize, usize)]) -> HashSet<usize> {
    let (&(factor, count), rest) = match factors.split_first() {
        Some(pair) => pair,
        None => return iter::once(1).collect(),
    };
    let rest_divisors = factors_to_divisors(rest);
    (1..=count)
        .flat_map(|num| {
            let multiplier = factor.pow(num as u32);
            rest_divisors
                .iter()
                .map(move |divisor| divisor * multiplier)
        })
        .chain(rest_divisors.iter().cloned())
        .collect()
}

fn get_factors(number: usize, primes: &[usize]) -> HashMultiSet<usize> {
    if number < 2 {
        return HashMultiSet::new();
    }
    let mut factors = HashMultiSet::new();
    let mut number = number;
    for &prime in primes {
        while divisible_by(number, prime) {
            factors.insert(prime);
            number /= prime;
        }
        if number == 1 {
            break;
        }
    }
    if number > 1 {
        factors.insert(number);
    }
    factors
}

fn divisible_by(number: usize, divisor: usize) -> bool {
    number % divisor == 0
}

fn primes_below(limit: usize) -> Vec<usize> {
    let mut is_composite = vec![false; limit];
    let mut primes = Vec::new();
    for num in (2..limit).map(|num| num) {
        if is_composite[num] {
            continue;
        }
        for index in (num..).step_by(num).take_while(|&x| x < limit) {
            is_composite[index] = true;
        }
        primes.push(num);
    }
    primes
}

fn iter_counts<T: Eq + std::hash::Hash>(
    multiset: &HashMultiSet<T>,
) -> impl Iterator<Item = (&T, usize)> {
    multiset
        .distinct_elements()
        .map(move |elem| (elem, multiset.count_of(elem)))
}

fn sum_of_divisors(num: usize, primes: &[usize]) -> usize {
    get_divisors(num, primes).iter().sum::<usize>()
}

fn is_abundant(num: usize, primes: &[usize]) -> bool {
    sum_of_divisors(num, primes) > num
}

fn is_sum_of_abundant_nums(
    num: usize,
    abundant_nums_vec: &[usize],
    abundant_nums_set: &HashSet<usize>,
) -> bool {
    abundant_nums_vec
        .iter()
        .take_while(|abundant_num| **abundant_num < num)
        .any(|abundant_num| abundant_nums_set.contains(&(num - abundant_num)))
}

fn solve() -> usize {
    let primes = primes_below(MAX_SUM_TWO_ABUNDANT_NUMS);
    let abundant_nums_vec = (1..=MAX_SUM_TWO_ABUNDANT_NUMS)
        .filter(|num| is_abundant(*num, &primes))
        .collect::<Vec<_>>();
    let abundant_nums_set = abundant_nums_vec.iter().cloned().collect::<HashSet<_>>();
    (1..=MAX_SUM_TWO_ABUNDANT_NUMS)
        .filter(|num| !is_sum_of_abundant_nums(*num, &abundant_nums_vec, &abundant_nums_set))
        .sum()
}

const MAX_SUM_TWO_ABUNDANT_NUMS: usize = 28_123;

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
const ANSWER: usize = 4_179_871;

#[test]
fn test() {
    assert_eq!(ANSWER, solve());
}
