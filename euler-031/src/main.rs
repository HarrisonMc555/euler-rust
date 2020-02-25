/*
In the United Kingdom the currency is made up of pound (£) and pence (p). There
are eight coins in general circulation:

1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).

It is possible to make £2 in the following way:

1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p

How many different ways can £2 be made using any number of coins?
*/

type Num = u32;

const COINS: [Num; 8] = [200, 100, 50, 20, 10, 5, 2, 1];
const TARGET: Num = 200;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    num_possibilities(&COINS, TARGET)
}

fn num_possibilities(nums: &[Num], target: Num) -> usize {
    if target == 0 {
        return 1;
    }
    let (num, rest) = match nums.split_first() {
        Some(nr) => nr,
        None => return 0,
    };
    let times_num_fits = target / num;
    (0..=times_num_fits)
        .map(|n| n * num)
        .map(|n| num_possibilities(rest, target - n))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer() {
        assert_eq!(solve(), 73682);
    }
}
