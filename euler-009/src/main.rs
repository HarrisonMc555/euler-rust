/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.  Find
the product abc.
*/

fn is_pythagorean_triplet(a: u64, b: u64, c: u64) -> bool {
    (a * a) + (b * b) == (c * c)
}

fn sum_triplets(sum: u64) -> impl Iterator<Item = (u64, u64, u64)> {
    (1..=sum-1)
        .flat_map(move |a| (a..=sum-a-1).map(move |b| (a, b, sum - a - b)))
}

fn solve(sum: u64) -> u64 {
    sum_triplets(sum)
        .filter(|&(a, b, c)| is_pythagorean_triplet(a, b, c))
        .map(|(a, b, c)| a * b * c)
        .next()
        .expect("no solution found")
}

const SUM: u64 = 1_000;

fn main() {
    println!("{}", solve(SUM));
}

const ANSWER: u64 = 31875000;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(SUM));
}
