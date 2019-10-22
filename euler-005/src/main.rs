/*
2520 is the smallest number that can be divided by each of the numbers from 1 to
10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the
numbers from 1 to 20?
 */

use multiset::HashMultiSet;

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

fn solve(limit: usize) -> usize {
    let all_factors = (2..=limit)
        .map(|num| get_factors(num))
        .fold(HashMultiSet::new(), |acc, factors| {
            acc.union(&factors).cloned().collect()
        });
    all_factors.iter().product()
}

const LIMIT: usize = 20;

fn main() {
    println!("{}", solve(LIMIT));
}

#[cfg(test)]
const ANSWER: usize = 232792560;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(LIMIT));
}
