/*
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143?
*/

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

fn get_factors(number: usize) -> Vec<usize> {
    let max_factor = (number as f64).sqrt() as usize;
    if number < 2 {
        return Vec::new();
    }
    let mut factors = Vec::new();
    let primes = primes_below(max_factor + 1);
    let mut number = number;
    for prime in primes {
        while divisible_by(number, prime) {
            factors.push(prime);
            number /= prime;
        }
        if number == 1 {
            break;
        }
    }
    if number > 1 {
        factors.push(number);
    }
    factors
}

fn divisible_by(number: usize, divisor: usize) -> bool {
    number % divisor == 0
}

fn solve(number: usize) -> usize {
    get_factors(number)
        .iter()
        .max()
        .copied()
        .expect("no prime factors")
}

const NUMBER: usize = 600_851_475_143;

fn main() {
    println!("{:?}", solve(NUMBER));
}

const ANSWER: usize = 6857;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(NUMBER));
}
