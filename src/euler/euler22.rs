/*
Using names.txt (right click and 'Save Link/Target As...'), a 46K text file
containing over five-thousand first names, begin by sorting it into alphabetical
order. Then working out the alphabetical value for each name, multiply this
value by its alphabetical position in the list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN, which is
worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would
obtain a score of 938 × 53 = 49714.

What is the total of all the name scores in the file?
*/

const LOWER_CASE_OFFSET: u8 = b'a' - 1;
const UPPER_CASE_OFFSET: u8 = b'A' - 1;

const FILE_CONTENTS: &str = include_str!("../../static/p022_names.txt");

fn alphabetical_value(c: char) -> u8 {
    if ('a'..='z').contains(&c) {
        return c as u8 - LOWER_CASE_OFFSET;
    } else if ('A'..='Z').contains(&c) {
        return c as u8 - UPPER_CASE_OFFSET;
    }
    0
}

fn word_value(word: &str) -> usize {
    word.chars().map(alphabetical_value).map(usize::from).sum()
}

fn normalize(entry: &str) -> &str {
    &entry[1..entry.len() - 1]
}

fn solve(contents: &str) -> usize {
    let mut names = contents.split(',').collect::<Vec<_>>();
    names.sort_unstable();
    names
        .iter()
        .map(|entry| normalize(entry))
        .enumerate()
        .map(|(i, name)| {
            let position = i + 1;
            let value = word_value(name);
            position * value
        })
        .sum()
}

pub fn main() {
    println!("{}", solve(FILE_CONTENTS));
}

#[cfg(test)]
const ANSWER: usize = 871198282;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(FILE_CONTENTS));
}
