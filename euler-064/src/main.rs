/*
All square roots are periodic when written as continued fractions and can be written in the form:

sqrt(N) = a_0 + 1 / (a_1 + 1 / (a_2 + 1 / (a_3 + ...))

For example, let us consider sqrt(23):

sqrt(23) = 4 + sqrt(23) - 4 = 4 + 1 / 1 / (sqrt(23) - 4) = 4 + 1 / (1 + (sqrt(23) - 3) / 7)

If we continue we would get the following expansion:

...

The process can be summarised as follows:

...

It can be seen that the sequence is repeating. For conciseness, we use the notation sqrt(23) = [4; (1, 3, 1, 8)], to
indicate that the block (1, 3, 1, 8) repeats indefinitely.

The first ten continued fraction representations of (irrational) square roots are:

...

Exactly four continued fractions, for N <= 13, have an odd period.

How many continued fractions for N <= 10,000 have an odd period?
 */

use std::cmp::Ordering;
use std::collections::HashMap;

const MAX: u32 = 10_000;

fn main() {
    let num_odd_periods = (2..=MAX)
        .map(expanded_sqrt)
        .filter_map(|sqrt| match sqrt {
            SqrtExpansionResult::Expanded(expanded) => Some(expanded),
            _ => None,
        })
        .map(|expanded| {
            if expanded.period_starting_index != 0 {
                println!("{:?}", expanded)
            }
            expanded
        })
        .map(|expanded| expanded.period_len())
        .filter(|len| is_odd(*len))
        .count();
    println!("{}", num_odd_periods);
}

#[derive(Debug, Eq, PartialEq)]
enum SqrtExpansionResult {
    Perfect(u32),
    Expanded(ExpandedSqrt),
}

#[derive(Debug, Eq, PartialEq)]
struct ExpandedSqrt {
    first: u32,
    expansion: Vec<u32>,
    period_starting_index: usize,
}

impl SqrtExpansionResult {
    pub fn perfect(sqrt: u32) -> Self {
        Self::Perfect(sqrt)
    }

    pub fn expanded(first: u32, expansion: Vec<u32>, period_starting_index: usize) -> Self {
        Self::Expanded(ExpandedSqrt::new(first, expansion, period_starting_index))
    }
}

impl ExpandedSqrt {
    pub fn new(first: u32, expansion: Vec<u32>, period_starting_index: usize) -> Self {
        Self {
            first,
            expansion,
            period_starting_index,
        }
    }

    pub fn period_len(&self) -> usize {
        self.expansion.len() - self.period_starting_index
    }
}

fn expanded_sqrt(n: u32) -> SqrtExpansionResult {
    let mut expansion = Vec::new();
    let mut seen = HashMap::new();
    let a_0 = approx_sqrt(n);
    let mut prev_numerator = 1;
    let mut prev_subtrahend = a_0;
    if a_0 * a_0 == n {
        return SqrtExpansionResult::perfect(a_0);
    }
    loop {
        let entry = (prev_numerator, prev_subtrahend);
        if let Some(index) = seen.remove(&entry) {
            if index != 0 {
                println!(
                    "sqrt({}) = [{}; {:?}] starting @ {}",
                    n, a_0, expansion, index
                );
            }
            return SqrtExpansionResult::expanded(a_0, expansion, index);
        }
        seen.insert(entry, expansion.len());
        let next_a =
            prev_numerator * (a_0 + prev_subtrahend) / (n - prev_subtrahend * prev_subtrahend);
        expansion.push(next_a);
        let next_numerator = (n - prev_subtrahend * prev_subtrahend) / prev_numerator;
        let next_subtrahend = next_a * next_numerator - prev_subtrahend;
        prev_numerator = next_numerator;
        prev_subtrahend = next_subtrahend;

        #[cfg(test)]
        if expansion.len() > 1_000 {
            panic!("Too much expansion");
        }
    }
}

fn approx_sqrt(n: u32) -> u32 {
    let mut min = 0;
    let mut max = n;
    let mut cur = n / 2;
    while min < max {
        match (cur * cur).cmp(&n) {
            Ordering::Equal => return cur,
            Ordering::Less => min = cur,
            Ordering::Greater => max = cur - 1,
        }
        cur = (min + max + 1) / 2;
    }
    cur
}

fn is_odd(n: usize) -> bool {
    n % 2 != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_approx_sqrt() {
        let test_cases = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 2),
            (9, 3),
            (10, 3),
            (24, 4),
            (25, 5),
            (25, 5),
            (31, 5),
            (35, 5),
            (36, 6),
            (37, 6),
            (99, 9),
            (100, 10),
        ];
        for (n, expected) in test_cases {
            let actual = approx_sqrt(n);
            assert_eq!(expected, actual, "sqrt({})", n);
        }
    }

    #[test]
    fn test_expanded_sqrt() {
        let test_cases = [
            (2, SqrtExpansionResult::expanded(1, vec![2], 0)),
            (3, SqrtExpansionResult::expanded(1, vec![1, 2], 0)),
            (4, SqrtExpansionResult::perfect(2)),
            (5, SqrtExpansionResult::expanded(2, vec![4], 0)),
            (6, SqrtExpansionResult::expanded(2, vec![2, 4], 0)),
            (7, SqrtExpansionResult::expanded(2, vec![1, 1, 1, 4], 0)),
            (8, SqrtExpansionResult::expanded(2, vec![1, 4], 0)),
            (9, SqrtExpansionResult::perfect(3)),
            (10, SqrtExpansionResult::expanded(3, vec![6], 0)),
            (11, SqrtExpansionResult::expanded(3, vec![3, 6], 0)),
            (12, SqrtExpansionResult::expanded(3, vec![2, 6], 0)),
            (13, SqrtExpansionResult::expanded(3, vec![1, 1, 1, 1, 6], 0)),
            (23, SqrtExpansionResult::expanded(4, vec![1, 3, 1, 8], 0)),
        ];
        for (n, expected) in test_cases {
            let actual = expanded_sqrt(n);
            assert_eq!(expected, actual, "sqrt({})", n);
        }
    }
}
