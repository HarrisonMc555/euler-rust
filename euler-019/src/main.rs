/*
You are given the following information, but you may prefer to do some research
for yourself.

  - 1 Jan 1900 was a Monday.
  - Thirty days has September,
    April, June and November.
    All the rest have thirty-one,
    Saving February alone,
    Which has twenty-eight, rain or shine.
    And on leap years, twenty-nine.
  - A leap year occurs on any year evenly divisible by 4, but not on a century
    unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1
Jan 1901 to 31 Dec 2000)?

*/

use chrono::{Datelike, NaiveDate, Weekday};

type Ymd = (i32, u32, u32);

struct DayIter {
    day: NaiveDate,
}

impl DayIter {
    fn new(start: NaiveDate) -> Self {
        DayIter { day: start }
    }
}

impl Iterator for DayIter {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.day.clone();
        self.day = self.day.succ();
        Some(prev)
    }
}

fn is_first_of_month(date: NaiveDate) -> bool {
    date.day() == 1
}

fn next_weekday(date: NaiveDate, weekday: Weekday) -> NaiveDate {
    DayIter::new(date)
        .find(|date| date.weekday() == weekday)
        .unwrap()
}

fn solve(start: Ymd, end: Ymd, weekday: Weekday) -> usize {
    let start = NaiveDate::from_ymd(start.0, start.1, start.2);
    let end = NaiveDate::from_ymd(end.0, end.1, end.2);
    let first_with_weekday = next_weekday(start, weekday);
    DayIter::new(first_with_weekday)
        .step_by(DAYS_IN_WEEK)
        .filter(|&date| is_first_of_month(date))
        .take_while(|day| *day < end)
        .count()
}

const DAYS_IN_WEEK: usize = 7;

const START_YEAR: i32 = 1901;
const START_MONTH: u32 = 1;
const START_DAY: u32 = 1;
const START: Ymd = (START_YEAR, START_MONTH, START_DAY);

const END_YEAR: i32 = 2000;
const END_MONTH: u32 = 12;
const END_DAY: u32 = 31;
const END: Ymd = (END_YEAR, END_MONTH, END_DAY);

const WEEKDAY: Weekday = Weekday::Sun;

fn main() {
    println!("{}", solve(START, END, WEEKDAY));
}

#[cfg(test)]
const ANSWER: usize = 171;

#[test]
fn test() {
    assert_eq!(ANSWER, solve(START, END, WEEKDAY));
}
