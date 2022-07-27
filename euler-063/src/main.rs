/*
The 5-digit number, 16807=7^5, is also a fifth power. Similarly, the 9-digit number, 134217728=8^9, is a ninth power.

How many n-digit positive integers exist which are also an nth power?
 */

use num_bigint::BigUint;
use num_traits::Zero;
use std::ops::DivAssign;

const DIGIT_BASE: u32 = 10;

// 10 ^ 1 has 2 digits
// 10 ^ 2 has 3 digits
// 10 ^ n always has n + 1 digits
// Any base greater than 10 will always have more digits than the exponent
const MAX_BASE: u32 = 10;

fn main() {
    let answer = (0..MAX_BASE).map(BigUint::from).flat_map(get_matching_power_num_digits).count();
    println!("{}", answer);
}

fn get_matching_power_num_digits(base: BigUint) -> impl Iterator<Item = u32> {
    (0..)
        .map(move |power| (power, num_digits(base.pow(power))))
        .skip_while(|(power, num_digits)| power < num_digits)
        .take_while(|(power, num_digits)| power == num_digits)
        .map(|(power, _)| power)
}

fn num_digits(mut num: BigUint) -> u32 {
    let mut count = 0;
    while num > BigUint::zero() {
        num.div_assign(DIGIT_BASE);
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(1, num_digits(1));
        assert_eq!(1, num_digits(2));
        assert_eq!(1, num_digits(9));
        assert_eq!(2, num_digits(10));
        assert_eq!(2, num_digits(11));
        assert_eq!(2, num_digits(50));
        assert_eq!(2, num_digits(99));
        assert_eq!(3, num_digits(100));
        assert_eq!(3, num_digits(101));
        assert_eq!(5, num_digits(12345));
        assert_eq!(8, num_digits(12345678));
        assert_eq!(13, num_digits(1234567890123));
    }

    fn num_digits(num: usize) -> u32 {
        super::num_digits(BigUint::from(num))
    }

    #[test]
    fn test_get_matching_power_num_digits() {
        let actual: Vec<_> = get_matching_power_num_digits(2).collect();
        let expected = vec![1];
        assert_eq!(expected, actual);

        let actual: Vec<_> = get_matching_power_num_digits(4).collect();
        let expected = vec![1, 2];
        assert_eq!(expected, actual);

        let actual: Vec<_> = get_matching_power_num_digits(5).collect();
        let expected = vec![1, 2, 3];
        assert_eq!(expected, actual);
    }

    fn get_matching_power_num_digits(num: usize) -> impl Iterator<Item = u32> {
        super::get_matching_power_num_digits(BigUint::from(num))
    }
}
