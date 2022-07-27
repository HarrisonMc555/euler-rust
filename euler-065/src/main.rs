/*
The square root of 2 can be written as an infinite continued fraction.

...

The infinite continued fraction can be written, sqrt(2) = [1; (2)], (2) indicates that 2 repeats ad infinitum. In a
similar way, sqrt(23) = [4; (1, 3, 1, 8)].

It turns out that the sequence of partial values of continued fractions for square roots provide the best rational
approximations. Let us consider the convergents for sqrt(2).

...

Hence the sequence of the first ten convergents for sqrt(2) are:

...

What is most surprising is that the important mathematical constant, e = [2; 1, 2, 1, 1, 4, 1, 1, 6, 1, ... , 1, 2k, 1, ...].

The first ten terms in the sequence of convergents for e are:

2, 3, 8/3, 11/4, 19/7, 87/32, 106/39, 193/71, 1264/465, 1457/536, ...

The sum of digits in the numerator of the 10th convergent is 1 + 4 + 5 + 7 = 17.

Find the sum of digits in the numerator of the 100th convergent of the continued fraction for e.
 */

use fraction::{BigUint, ToPrimitive, Zero};

type Fraction = fraction::GenericFraction<BigUint>;

const NUM_CONVERGENT: u64 = 100;
const BASE: u64 = 10;

fn main() {
    let expansion = (1..NUM_CONVERGENT).map(expansion_e).rev();
    let convergent = get_convergent(2, expansion);
    let numer = convergent.numer().expect("valid fraction");
    let sum = sum_digits(numer.clone(), BASE);
    println!("{}", sum);
}

fn get_convergent<I>(first: u64, expansion_from_end: I) -> Fraction
where
    I: Iterator<Item = u64>,
{
    let mut cur = Fraction::infinity();
    for n in expansion_from_end {
        cur = Fraction::from(n) + cur.recip();
    }
    Fraction::from(first) + cur.recip()
}

fn expansion_e(index: u64) -> u64 {
    if index == 0 {
        2
    } else if index % 3 == 2 {
        let k = index / 3 + 1;
        2 * k
    } else {
        1
    }
}

fn sum_digits(mut n: BigUint, base: u64) -> u64 {
    let base = BigUint::from(base);
    let mut sum = BigUint::zero();
    while n > BigUint::zero() {
        let digit = &n % &base;
        sum += digit;
        n /= &base;
    }
    sum.to_u64().expect("Sum of digits is too large")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_e_expansion() {
        let expected = vec![
            2, 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, 1, 1, 10, 1, 1, 12, 1, 1, 14, 1, 1, 16, 1, 1, 18,
            1, 1, 20, 1, 1, 22, 1, 1, 24, 1,
        ];
        let actual: Vec<_> = ExpansionE::new().take(expected.len()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_convergent() {
        let test_cases = [
            Fraction::from(1),
            Fraction::new(3u64, 2u64),
            Fraction::new(7u64, 5u64),
            Fraction::new(17u64, 12u64),
            Fraction::new(41u64, 29u64),
            Fraction::new(99u64, 70u64),
            Fraction::new(239u64, 169u64),
            Fraction::new(577u64, 408u64),
            Fraction::new(1393u64, 985u64),
            Fraction::new(3363u64, 2378u64),
        ];
        for (i, expected) in test_cases.into_iter().enumerate() {
            let expanded = std::iter::repeat(2).take(i);
            let actual = get_convergent(1, expanded);
            assert_eq!(expected, actual, "Convergent with {} terms", i);
        }
    }

    #[test]
    fn test_sum_digits() {
        assert_eq!(1, sum_digits(1));
        assert_eq!(3, sum_digits(3));
        assert_eq!(9, sum_digits(72));
        assert_eq!(4, sum_digits(13));
        assert_eq!(27, sum_digits(999));
        assert_eq!(8, sum_digits(800));
    }

    fn sum_digits(num: u64) -> u64 {
        super::sum_digits(BigUint::from(num), base)
    }
}
