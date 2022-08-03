/*
A unit fraction contains 1 in the numerator. The decimal representation of the
unit fractions with denominators 2 to 10 are given:

1/2  = 0.5
1/3  = 0.(3)
1/4  = 0.25
1/5  = 0.2
1/6  = 0.1(6)
1/7  = 0.(142857)
1/8  = 0.125
1/9  = 0.(1)
1/10 = 0.1

Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be
seen that 1/7 has a 6-digit recurring cycle.

Find the value of d < 1000 for which 1/d contains the longest recurring cycle in
its decimal fraction part.
*/

type Digit = u8;
const DEFAULT_BASE: u32 = 10;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Digits {
    leading: Vec<Digit>,
    repeating: Vec<Digit>,
}

pub fn main() {
    println!("{}", solve());
}

fn solve() -> u32 {
    const MAX: u32 = 1000;
    (1..MAX)
        .max_by_key(|d| Digits::one_over(*d).repeating.len())
        .unwrap()
}

impl Digits {
    fn new(leading: Vec<Digit>, repeating: Vec<Digit>) -> Self {
        Digits { leading, repeating }
    }

    fn one_over(den: u32) -> Digits {
        Digits::one_over_base(den, DEFAULT_BASE)
    }

    fn one_over_base(den: u32, base: u32) -> Digits {
        if den == 0 {
            panic!("Must be greater than one")
        }
        let mut digits_rems: Vec<(Digit, u32)> = Vec::new();
        let mut num = base;
        loop {
            if num == 0 {
                return Digits::new(from_digits_rems(&digits_rems), Vec::new());
            }
            let (quot, rem) = (num / den, num % den);
            let quot = quot as Digit;
            num = rem * base;
            if let Some(index) = digits_rems.iter().position(|&(_, r)| r == rem) {
                let (match_digit, _) = digits_rems[index];
                return if match_digit == quot {
                    Digits::new(
                        from_digits_rems(&digits_rems[..index]),
                        from_digits_rems(&digits_rems[index..]),
                    )
                } else {
                    digits_rems.push((quot, rem));
                    Digits::new(
                        from_digits_rems(&digits_rems[..index + 1]),
                        from_digits_rems(&digits_rems[index + 1..]),
                    )
                }
            }
            digits_rems.push((quot, rem));
        }
    }
}

fn from_digits_rems(digits_rems: &[(Digit, u32)]) -> Vec<Digit> {
    digits_rems.iter().map(|&(d, _)| d).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two() {
        assert_eq!(Digits::new(vec![5], vec![]), Digits::one_over(2));
    }

    #[test]
    fn three() {
        assert_eq!(Digits::new(vec![], vec![3]), Digits::one_over(3));
    }

    #[test]
    fn four() {
        assert_eq!(Digits::new(vec![2, 5], vec![]), Digits::one_over(4));
    }

    #[test]
    fn five() {
        assert_eq!(Digits::new(vec![2], vec![]), Digits::one_over(5));
    }

    #[test]
    fn six() {
        assert_eq!(Digits::new(vec![1], vec![6]), Digits::one_over(6));
    }

    #[test]
    fn seven() {
        assert_eq!(
            Digits::new(vec![], vec![1, 4, 2, 8, 5, 7]),
            Digits::one_over(7)
        );
    }

    #[test]
    fn eight() {
        assert_eq!(Digits::new(vec![1, 2, 5], vec![]), Digits::one_over(8));
    }

    #[test]
    fn nine() {
        assert_eq!(Digits::new(vec![], vec![1]), Digits::one_over(9));
    }

    #[test]
    fn ten() {
        assert_eq!(Digits::new(vec![1], vec![]), Digits::one_over(10));
    }

    #[test]
    fn answer() {
        assert_eq!(983, solve());
    }
}
