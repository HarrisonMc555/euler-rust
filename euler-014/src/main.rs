/*
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains
10 terms. Although it has not been proved yet (Collatz Problem), it is thought
that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
*/

fn next_collatz(number: usize) -> usize {
    if number % 2 == 0 {
        number / 2
    } else {
        3 * number + 1
    }
}

fn find_collatz_len(collatz_array: &mut [Option<usize>], number: usize) {
    let mut sequence = Vec::new();
    let mut number = number;
    loop {
        match collatz_array.get(number) {
            Some(Some(_)) => break,
            _ => (),
        }
        sequence.push(number);
        number = next_collatz(number);
    }
    let remaining_length = collatz_array[number].expect("ended with None");
    for (i, &n) in sequence.iter().rev().enumerate() {
        if let Some(count) = collatz_array.get_mut(n) {
            *count = Some(remaining_length + i + 1)
        }
    }
}

fn solve(limit: usize) -> usize {
    let mut collatz_array = vec![None; limit];
    collatz_array[1] = Some(1);
    for number in 1..limit {
        find_collatz_len(&mut collatz_array, number);
    }
    collatz_array
        .iter()
        .enumerate()
        .filter_map(|(index, &option)| option.map(|count| (index, count)))
        .max_by_key(|&(_, count)| count)
        .map(|(index, _)| index)
        .expect("limit too small")
}

const LIMIT: usize = 1_000_000;

fn main() {
    println!("{}", solve(LIMIT));
}

#[cfg(test)]
const ANSWER: usize = 837799;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(LIMIT));
}
