/*
Starting in the top left corner of a 2×2 grid, and only being able to move to
the right and down, there are exactly 6 routes to the bottom right corner.

*-*-*  *-*-   *-*-   *- -   *- -   *- -
| | |  | | |  | | |  | | |  | | |  | | |
 - -*   -*-*   -*-   *-*-*  *-*-   *- -
| | |  | | |  | | |  | | |  | | |  | | |
 - -*   - -*   -*-*   - -*   -*-*  *-*-*

How many such routes are there through a 20×20 grid?
*/

use num_rational::Rational64;

fn solve(width: i64) -> i64 {
    (1..=width)
        .fold(Rational64::new(1i64, 1), |acc, i| {
            acc * Rational64::new(width + i, i)
        })
        .to_integer()
}

const WIDTH: i64 = 20;

fn main() {
    println!("{}", solve(WIDTH));
}
