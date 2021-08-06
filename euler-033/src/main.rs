/*
The fraction 49/98 is a curious fraction, as an inexperienced mathematician in
attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is
correct, is obtained by cancelling the 9s.

We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

There are exactly four non-trivial examples of this type of fraction, less than
one in value, and containing two digits in the numerator and denominator.

If the product of these four fractions is given in its lowest common terms, find
the value of the denominator.
*/

fn main() {
    // println!("{:?}", sieve(100).collect::<Vec<_>>());
    let primes = sieve(100).collect::<Vec<_>>();
    for i in 2..=100 {
        println!("{}: {:?}", i, factors(i, &primes));
    }
}

fn solve() -> usize {
    let mut nums = Vec::new();
    for (na, nb) in two_decimal_digits() {
        for (da, db) in two_decimal_digits() {
            if !matches((na, nb), (da, db)) {
                continue;
            }
            
            //
        }
    }
    0
}

fn matches((na, nb): (usize, usize), (da, db): (usize, usize)) -> bool {
    (na == da && na != 0) || (na == db && na != 0) || (nb == da && nb != 0) || (nb == db && nb != 0)
}

fn two_decimal_digits() -> impl Iterator<Item = (usize, usize)> {
    decimal_digits().flat_map(move |a| decimal_digits().map(move |b| (a, b)))
}

fn decimal_digits() -> impl Iterator<Item = usize> {
    0..10
}

fn f64_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.000001
}

fn factors(num: usize, primes: &[usize]) -> Vec<usize> {
    if num <= 1 {
        return Vec::new();
    }
    let mut cur = num;
    let max_factor = (num as f64).sqrt() as usize;
    let mut num_factors = Vec::new();
    for &prime in primes.iter().take_while(|&&prime| prime <= max_factor) {
        if cur == 1 {
            return num_factors;
        }
        while is_divisible_by(cur, prime) {
            num_factors.push(prime);
            cur /= prime;
        }
    }
    if cur > 1 {
        // If the list of primes is too small, this may not be prime. However,
        // we have no way of knowing that without calculating primes
        // ourselves. We will trust the caller to give us enough primes.
        num_factors.push(cur);
    }
    num_factors
}

fn is_divisible_by(num: usize, divisor: usize) -> bool {
    num % divisor == 0
}

fn sieve(max: usize) -> impl Iterator<Item = usize> {
    let mut marks = vec![false; max + 1];
    for i in 2..=max {
        if marks[i] {
            continue;
        }
        for j in (2 * i..).step_by(i).take_while(|&j| j <= max) {
            marks[j] = true;
        }
    }
    marks
        .into_iter()
        .enumerate()
        .skip_while(|(i, _)| *i < 2)
        .filter(|(_, marked)| !marked)
        .map(|(i, _)| i)
}
