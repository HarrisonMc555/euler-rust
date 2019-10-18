/*
By starting at the top of the triangle below and moving to adjacent numbers on
the row below, the maximum total from top to bottom is 23.

     *3*
   *7*  4
  2  *4*  6
8   5  *9*  3

That is, 3 + 7 + 4 + 9 = 23.

Find the maximum total from top to bottom of the triangle below:

                            75
                          95  64
                        17  47  82
                      18  35  87  10
                    20  04  82  47  65
                  19  01  23  75  03  34
                88  02  77  73  07  63  67
              99  65  04  28  06  16  70  92
            41  41  26  56  83  40  80  70  33
          41  48  72  33  47  32  37  16  94  29
        53  71  44  65  25  43  91  52  97  51  14
      70  11  33  28  77  73  17  78  39  68  17  57
    91  71  52  38  17  14  91  43  58  50  27  29  48
  63  66  04  68  89  53  67  30  73  16  69  87  40  31
04  62  98  27  23  09  70  98  73  93  38  53  60  04  23

NOTE: As there are only 16384 routes, it is possible to solve this problem by
trying every route. However, Problem 67, is the same challenge with a triangle
containing one-hundred rows; it cannot be solved by brute force, and requires a
clever method! ;o)
*/

fn solve(triangle: &[Vec<u64>]) -> u64 {
    let mut best_nums: Vec<u64> = triangle[0].clone();
    for row in triangle[1..].into_iter() {
        let best_len = best_nums.len();
        let row_len = row.len();
        assert_eq!(row_len, best_len + 1);
        let mut next_best_nums: Vec<u64> = Vec::new();
        next_best_nums.push(best_nums[0] + row[0]);
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
    line.split(", ").map(str::parse).collect()
}

fn parse_triangle_strings(triangle_strings: &[&str]) -> Vec<Vec<u64>> {
    triangle_strings
        .iter()
        .map(|line| line_to_nums(line).expect("invalid number"))
        .collect()
}

const TRIANGLE_STRINGS: [&str; 15] = [
    "75",
    "95, 64",
    "17, 47, 82",
    "18, 35, 87, 10",
    "20, 04, 82, 47, 65",
    "19, 01, 23, 75, 03, 34",
    "88, 02, 77, 73, 07, 63, 67",
    "99, 65, 04, 28, 06, 16, 70, 92",
    "41, 41, 26, 56, 83, 40, 80, 70, 33",
    "41, 48, 72, 33, 47, 32, 37, 16, 94, 29",
    "53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14",
    "70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57",
    "91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48",
    "63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31",
    "04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23",
];

// const EXAMPLE_TRIANGLE_STRINGS: [&str; 4] = ["3", "7, 4", "2, 4, 6", "8, 5, 9, 3"];

fn main() {
    println!("{}", solve(&parse_triangle_strings(&TRIANGLE_STRINGS)));
    // println!("{}", solve(&parse_triangle_strings(&EXAMPLE_TRIANGLE_STRINGS)));
}

#[cfg(test)]
const ANSWER: u64 = 1074;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(&parse_triangle_strings(&TRIANGLE_STRINGS)));
}
