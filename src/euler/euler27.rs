/*
Euler discovered the remarkable quadratic formula:

n^2+n+41

It turns out that the formula will produce 40 primes for the consecutive
integer values 0≤n≤39. However, when n=40,40^2+40+41=40(40+1)+41 is divisible by
41, and certainly when n=41,41^2+41+41 is clearly divisible by 41.

The incredible formula n^2−79n+1601 was discovered, which produces 80 primes for
the consecutive values 0≤n≤79. The product of the coefficients, −79 and 1601, is
−126479.

Considering quadratics of the form:

n^2+an+b, where |a|<1000 and |b|≤1000

where |n| is the modulus/absolute value of n e.g. |11|=11 and |−4|=4 Find the
product of the coefficients, a and b, for the quadratic expression that produces
the maximum number of primes for consecutive values of n, starting with n=0.
*/

use primal::Sieve;
use std::convert::TryFrom;

pub fn main() {
    println!("{}", solve());
}

fn solve() -> Num {
    const MAX: Num = 1000;
    let a_iter = (-MAX..MAX).filter(|&n| n != 0);
    let b_iter = -MAX..MAX;
    // n^2 + a*n + b where n = a = b = MAX
    const MAX_NUM: Num = MAX * MAX + MAX * MAX + MAX;
    let sieve = Sieve::new(MAX_NUM as usize);
    let formula = iproduct!(a_iter, b_iter)
        .map(Formula::from_tuple)
        .max_by_key(|f| f.count_primes(&sieve))
        .unwrap();
    formula.a * formula.b
}

type Num = i32;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Formula {
    a: Num,
    b: Num,
}

impl Formula {
    fn from_tuple((a, b): (Num, Num)) -> Formula {
        Formula { a, b }
    }

    fn as_tuple(&self) -> (Num, Num) {
        (self.a, self.b)
    }

    fn nums(&self) -> impl Iterator<Item = u64> + '_ {
        let (a, b) = self.as_tuple();
        (0..)
            .map(move |n| n * n + a * n + b)
            .flat_map(u64::try_from)
    }

    fn count_primes(&self, sieve: &Sieve) -> usize {
        self.nums()
            .take_while(|&n| sieve.is_prime(n as usize))
            .count()
    }
}
