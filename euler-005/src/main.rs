/*
2520 is the smallest number that can be divided by each of the numbers from 1 to
10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the
numbers from 1 to 20?
 */

use multiset::HashMultiSet;
use std::cmp;

fn primes_below(limit: usize) -> Vec<usize> {
    let mut is_composite = vec![false; limit];
    let mut primes = Vec::new();
    for num in 2..limit {
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

fn get_factors(number: usize) -> HashMultiSet<usize> {
    let max_factor = (number as f64).sqrt() as usize;
    if number < 2 {
        return HashMultiSet::new();
    }
    let mut factors = HashMultiSet::new();
    let primes = primes_below(max_factor + 1);
    let mut number = number;
    for prime in primes {
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

fn count_iter<'a, K: 'a>(multiset: &'a HashMultiSet<K>) -> CountIter<'a, K>
where
    K: Eq,
    K: std::hash::Hash,
{
    CountIter {
        iter: multiset.distinct_elements(),
        multiset: multiset,
    }
}

struct CountIter<'a, K: 'a>
where
    K: Eq,
    K: std::hash::Hash,
{
    iter: std::collections::hash_map::Keys<'a, K, usize>,
    multiset: &'a HashMultiSet<K>,
}

impl<'a, K> Iterator for CountIter<'a, K>
where
    K: Eq,
    K: std::hash::Hash,
{
    type Item = (&'a K, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|key| (key, self.multiset.count_of(key)))
    }
}

fn union<K>(multiset1: &HashMultiSet<K>, multiset2: &HashMultiSet<K>) -> HashMultiSet<K>
where
    K: Eq,
    K: std::hash::Hash,
    K: std::fmt::Debug,
    K: Clone,
{
    let mut result = HashMultiSet::new();
    for (key, count1) in count_iter(multiset1) {
        let count2 = multiset2.count_of(key);
        result.insert_times(key.clone(), cmp::max(count1, count2));
    }
    for (key, count2) in count_iter(multiset2) {
        if !result.contains(key) {
            result.insert_times(key.clone(), count2);
        }
    }
    result
}

fn solve(limit: usize) -> usize {
    let all_factors = (2..=limit)
        .map(|num| get_factors(num))
        .fold(HashMultiSet::new(), |acc, factors| union(&acc, &factors));
    all_factors.iter().product()
}

const LIMIT: usize = 20;

fn main() {
    println!("{}", solve(LIMIT));
}

const ANSWER: usize = 232792560;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(LIMIT));
}
