/*
By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top
to bottom is 23.

     *3*
   *7*  4
  2  *4*  6
8   5  *9*  3

That is, 3 + 7 + 4 + 9 = 23.

Find the maximum total from top to bottom in triangle.txt (right click and 'Save Link/Target As...'), a 15K text file
containing a triangle with one-hundred rows.

NOTE: This is a much more difficult version of Problem 18. It is not possible to try every route to solve this problem,
as there are 299 altogether! If you could check one trillion (1012) routes every second it would take over twenty
billion years to check them all. There is an efficient algorithm to solve it. ;o)
 */

const FILE_CONTENTS: &str = include_str!("../../static/p067_triangle.txt");

fn solve(triangle: &[Vec<u64>]) -> u64 {
    let mut best_nums: Vec<u64> = triangle[0].clone();
    for row in triangle[1..].iter() {
        let best_len = best_nums.len();
        let row_len = row.len();
        assert_eq!(row_len, best_len + 1);
        let mut next_best_nums: Vec<u64> = vec![best_nums[0] + row[0]];
        next_best_nums.extend(
            best_nums
                .windows(2)
                .map(|pair| pair.iter().max().unwrap())
                .zip(row[1..row_len - 1].iter())
                .map(|(best, next)| best + next),
        );
        next_best_nums.push(best_nums[best_len - 1] + row[row_len - 1]);
        best_nums = next_best_nums;
    }
    best_nums.iter().max().cloned().unwrap()
}

fn line_to_nums(line: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    line.split(" ").map(str::parse).collect()
}

fn parse_triangle_string(triangle_sting: &str) -> Vec<Vec<u64>> {
    triangle_sting
        .lines()
        .map(|line| line_to_nums(line).expect("invalid number"))
        .collect()
}

pub fn main() {
    println!("{}", solve(&parse_triangle_string(&FILE_CONTENTS)));
}

#[cfg(test)]
const ANSWER: u64 = 7273;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(&parse_triangle_string(&FILE_CONTENTS)));
}
