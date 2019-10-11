/*
The sequence of triangle numbers is generated by adding the natural numbers. So
the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten
terms would be:

1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

Let us list the factors of the first seven triangle numbers:

 1: 1
 3: 1, 3
 6: 1, 2, 3, 6
10: 1, 2, 5, 10
15: 1, 3, 5, 15
21: 1, 3, 7, 21
28: 1, 2, 4, 7, 14, 28

We can see that 28 is the first triangle number to have over five divisors.

What is the value of the first triangle number to have over five hundred
divisors?
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

fn num_divisors(number: usize, primes: &[usize]) -> usize {
    let factors = get_factors(number, primes);
    count_iter(&factors).map(|(_, count)| count + 1).product()
}

struct TriangleNumbers {
    number: usize,
    sum: usize,
}

impl TriangleNumbers {
    fn new() -> Self {
        TriangleNumbers { number: 1, sum: 0 }
    }
}

impl Iterator for TriangleNumbers {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.sum += self.number;
        self.number += 1;
        Some(self.sum)
    }
}

fn solve(min_num_divisors: usize) -> usize {
    let primes = primes_below(min_num_divisors * 100);
    TriangleNumbers::new()
        .find(|&number| num_divisors(number, &primes) > min_num_divisors)
        .expect("no solution found")
}

const MIN_NUM_DIVISORS: usize = 500;

fn main() {
    println!("{}", solve(MIN_NUM_DIVISORS));
}
