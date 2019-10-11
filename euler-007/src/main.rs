/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that
the 6th prime is 13.

What is the 10 001st prime number?
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

// fn get_factors(number: usize) -> Vec<usize> {
//     let max_factor = (number as f64).sqrt() as usize;
//     if number < 2 {
//         return Vec::new();
//     }
//     let mut factors = Vec::new();
//     let primes = primes_below(max_factor + 1);
//     let mut number = number;
//     for prime in primes {
//         while divisible_by(number, prime) {
//             factors.push(prime);
//             number /= prime;
//         }
//         if number == 1 {
//             break;
//         }
//     }
//     if number > 1 {
//         factors.push(number);
//     }
//     factors
// }

// fn divisible_by(number: usize, divisor: usize) -> bool {
//     number % divisor == 0
// }

const FACTOR: usize = 10;

fn estimate_nth_prime(nth: usize) -> usize {
    nth * FACTOR
}

fn nth_prime(nth: usize) -> usize {
    let mut limit = estimate_nth_prime(nth);
    loop {
        let primes = primes_below(limit);
        match primes.get(nth - 1) {
            Some(prime) => return *prime,
            None => {
                limit *= FACTOR;
            }
        }
    }
}

fn solve(nth: usize) -> usize {
    nth_prime(nth)
}

const NTH: usize = 10_001;

fn main() {
    println!("{}", solve(NTH));
}
