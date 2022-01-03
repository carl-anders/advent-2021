use super::day::Day;
use std::collections::HashMap;

pub struct Day10;
impl Day for Day10 {
    type Parsed = Vec<String>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input.collect()
    }

    fn first(lines: Vec<String>) -> String {
        let brace_map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
        let score_map = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
        let mut score = 0;
        for line in lines {
            let mut openers = Vec::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => openers.push(c),
                    _ => {
                        if brace_map[&openers.pop().unwrap()] != c {
                            score += score_map[&c];
                            break;
                        }
                    }
                }
            }
        }
        score.to_string()
    }

    fn second(lines: Vec<String>) -> String {
        let brace_map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
        let score_map = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
        let mut scores: Vec<i64> = Vec::new();
        'outer: for line in lines {
            let mut openers = Vec::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => openers.push(c),
                    _ => {
                        if brace_map[&openers.pop().unwrap()] != c {
                            continue 'outer;
                        }
                    }
                }
            }
            scores.push(
                openers
                    .iter()
                    .rev()
                    .fold(0, |acc, b| acc * 5 + score_map[b]),
            );
        }
        scores.sort_unstable();
        scores[scores.len() / 2].to_string()
    }
}
