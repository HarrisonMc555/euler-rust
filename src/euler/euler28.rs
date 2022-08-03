/*
Starting with the number 1 and moving to the right in a clockwise direction a 5
by 5 spiral is formed as follows:

*21*  22   23   24  *25*
 20  * 7*   8  * 9*  10
 19    6  * 1*   2   11
 18  * 5*   4  * 3*  12
*17*  16   15   14  *13*

It can be verified that the sum of the numbers on the diagonals is 101.

What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed
in the same way?
*/

const WIDTH: Num = 1001;

type Num = u64;

pub fn main() {
    println!("{}", sum_diagonals(WIDTH).unwrap());
}

fn sum_diagonals(final_width: Num) -> Option<Num> {
    if final_width % 2 != 1 {
        return None;
    }
    let mut cur = 1;
    let mut sum = 1;
    for width in (2..final_width).step_by(2) {
        sum += (1..=4).map(|n| cur + width * n).sum::<Num>();
        cur += 4 * width;
    }
    Some(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(sum_diagonals(5), Some(101));
    }

    #[test]
    fn answer() {
        assert_eq!(sum_diagonals(WIDTH), Some(669171001));
    }
}
