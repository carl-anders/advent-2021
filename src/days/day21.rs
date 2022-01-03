use std::collections::HashMap;

use super::day::Day;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Player {
    pos: i32,
    score: i32,
}
impl Player {
    pub fn go(&mut self, num: i32) {
        self.pos = (self.pos + num - 1) % 10 + 1;
        self.score += self.pos;
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Universe {
    players: [Player; 2],
}
impl Universe {
    pub fn has_won(&self) -> Option<usize> {
        for i in 0..self.players.len() {
            if self.players[i].score >= 21 {
                return Some(i);
            }
        }
        None
    }
}

pub struct Day21;
impl Day for Day21 {
    type Parsed = [Player; 2];

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input
            .map(|s| s.chars().last().unwrap().to_digit(10).unwrap() as i32)
            .map(|pos| Player { pos, score: 0 })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn first(mut players: Self::Parsed) -> String {
        let mut dice_num = 100;
        let mut throw = 0;

        loop {
            for player in 0..2 {
                let mut sum = 0;
                for _ in 0..3 {
                    dice_num = (dice_num % 100) + 1;
                    throw += 1;
                    sum += dice_num;
                }
                players[player].go(sum);
                if players[player].score >= 1000 {
                    let loser = (player + 1) % 2;
                    let points = players[loser].score * throw;
                    return points.to_string();
                }
            }
        }
    }

    fn second(players: Self::Parsed) -> String {
        let chance_map = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

        let mut wins: [i64; 2] = [0, 0];
        let mut universes = HashMap::new();
        universes.insert(Universe { players }, 1);

        while !universes.is_empty() {
            for player in 0..2 {
                let mut new_universes = HashMap::new();
                for (universe, num) in &universes {
                    for chance in chance_map {
                        let mut new = *universe;
                        new.players[player].go(chance.0);
                        match new.has_won() {
                            Some(i) => {
                                wins[i] += num * chance.1;
                            }
                            _ => {
                                *new_universes.entry(new).or_insert(0) += num * chance.1;
                            }
                        }
                    }
                }
                universes = new_universes;
            }
        }
        wins.iter().max().unwrap().to_string()
    }
}
