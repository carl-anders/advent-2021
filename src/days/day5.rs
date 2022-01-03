use itertools::{EitherOrBoth, Itertools};

use super::day::Day;
use std::collections::HashMap;

fn range(from: i32, to: i32) -> Box<dyn Iterator<Item = i32>> {
    if from > to {
        Box::new((to..=from).rev())
    } else {
        Box::new(from..=to)
    }
}

fn zip_default_and_for_each<F>(
    a: Box<dyn Iterator<Item = i32>>,
    a_default: i32,
    b: Box<dyn Iterator<Item = i32>>,
    b_default: i32,
    mut for_each: F,
) where
    F: FnMut((i32, i32)),
{
    a.zip_longest(b).for_each(|e| match e {
        EitherOrBoth::Both(ia, ib) => for_each((ia, ib)),
        EitherOrBoth::Left(ia) => for_each((ia, b_default)),
        EitherOrBoth::Right(ib) => for_each((a_default, ib)),
    });
}

pub struct Day5;
impl Day for Day5 {
    type Parsed = Vec<Vec<Vec<i32>>>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input
            .map(|line| {
                line.split(" -> ")
                    .map(|p| {
                        p.split(',')
                            .map(|value| value.parse::<i32>().ok().unwrap())
                            .collect()
                    })
                    .collect()
            })
            .collect()
    }
    fn first(positions: Self::Parsed) -> String {
        let mut map = HashMap::new();
        for pos in positions {
            if pos[0][1] == pos[1][1] || pos[0][0] == pos[1][0] {
                zip_default_and_for_each(
                    range(pos[0][0], pos[1][0]),
                    pos[0][0],
                    range(pos[0][1], pos[1][1]),
                    pos[0][1],
                    |p| *map.entry(p).or_insert(0) += 1,
                );
            }
        }
        map.retain(|_, v| *v > 1);
        map.keys().len().to_string()
    }

    fn second(positions: Self::Parsed) -> String {
        let mut map = HashMap::new();
        for pos in positions {
            zip_default_and_for_each(
                range(pos[0][0], pos[1][0]),
                pos[0][0],
                range(pos[0][1], pos[1][1]),
                pos[0][1],
                |p| *map.entry(p).or_insert(0) += 1,
            );
        }
        map.retain(|_, v| *v > 1);
        map.keys().len().to_string()
    }
}
