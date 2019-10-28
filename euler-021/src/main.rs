/*
Let d(n) be defined as the sum of proper divisors of n (numbers less than n
which divide evenly into n).  If d(a) = b and d(b) = a, where a ≠ b, then a and
b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55
and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and
142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.
*/

use multiset::HashMultiSet;
use std::collections::{HashMap, HashSet};
use std::iter;

fn sum_of_proper_divisors(number: usize, primes: &[usize]) -> usize {
    get_divisors(number, &primes).iter().sum()
}

fn sum_of_proper_divisors_naive(number: usize) -> usize {
    (1..number)
        .filter(|divisor| divisible_by(number, *divisor))
        .sum()
}

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

fn is_amicable_number(number: usize, map: &HashMap<usize, usize>) -> bool {
    let other = map
        .get(&number)
        .cloned()
        .unwrap_or_else(|| sum_of_proper_divisors_naive(number));
    if number == other {
        return false;
    }
    map.get(&other)
        .cloned()
        .unwrap_or_else(|| sum_of_proper_divisors_naive(other))
        == number
}

fn solve(limit: usize) -> usize {
    let primes = primes_below(limit);
    let map = (2..limit)
        .map(|number| (number, sum_of_proper_divisors(number, &primes)))
        .collect::<HashMap<_, _>>();
    (2..limit)
        .filter(|number| is_amicable_number(*number, &map))
        .sum()
}

const LIMIT: usize = 10_000;

fn main() {
    println!("{}", solve(LIMIT));
}

#[cfg(test)]
const ANSWER: usize = 31_626;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(LIMIT));
}
