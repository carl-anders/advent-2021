use std::collections::HashMap;

use itertools::Itertools;

use super::day::Day;

#[derive(Debug, Clone)]
pub struct Manual {
    counts: HashMap<(char, char), usize>,
    instructions: HashMap<(char, char), char>,
    last: char,
}

pub struct Day14;
impl Day for Day14 {
    type Parsed = Manual;

    fn parse(mut input: impl Iterator<Item = String>) -> Self::Parsed {
        let first = input.next().unwrap();
        Manual {
            counts: first.chars().tuple_windows().counts(),
            instructions: input
                .skip(1)
                .map(|l| {
                    l.split_once(" -> ")
                        .map(|(l, r)| {
                            (
                                (l.chars().next().unwrap(), l.chars().nth(1).unwrap()),
                                r.chars().next().unwrap(),
                            )
                        })
                        .unwrap()
                })
                .collect(),
            last: first.chars().last().unwrap(),
        }
    }

    fn first(data: Self::Parsed) -> String {
        solve(data, 10).to_string()
    }

    fn second(data: Self::Parsed) -> String {
        solve(data, 40).to_string()
    }
}

fn solve(data: Manual, times: i32) -> usize {
    let mut counts = data.counts;
    for _ in 1..=times {
        let mut new_counts = HashMap::new();
        for (pair, num) in &counts {
            let replace = data.instructions[pair];
            *new_counts.entry((pair.0, replace)).or_insert(0) += num;
            *new_counts.entry((replace, pair.1)).or_insert(0) += num;
        }
        counts = new_counts;
    }
    let mut character_counts = HashMap::new();
    for (pair, num) in &counts {
        *character_counts.entry(pair.0).or_insert(0) += num;
    }
    *character_counts.entry(data.last).or_default() += 1;
    match character_counts.values().minmax() {
        itertools::MinMaxResult::MinMax(min, max) => max - min,
        _ => unreachable!(),
    }
}
