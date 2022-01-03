use super::day::Day;
use pathfinding::directed::dijkstra::dijkstra;
use smallvec::{smallvec, SmallVec};

pub struct Day15;
impl Day for Day15 {
    type Parsed = Vec<Vec<u32>>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input
            .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect()
    }

    fn first(risks: Self::Parsed) -> String {
        explore(&risks).to_string()
    }

    fn second(risks: Self::Parsed) -> String {
        let mut risks_5 = Vec::new();
        for y in 0..(risks.len() * 5) {
            risks_5.push(Vec::new());
            let y_quot = y / risks.len();
            let y_rem = y % risks.len();
            for x in 0..(risks[y_rem].len() * 5) {
                let x_quot = x / risks[y_rem].len();
                let x_rem = x % risks[y_rem].len();
                risks_5[y].push((risks[y_rem][x_rem] + (y_quot + x_quot) as u32 - 1) % 9 + 1);
            }
        }
        explore(&risks_5).to_string()
    }
}

fn explore(risks: &[Vec<u32>]) -> u32 {
    let goal: (usize, usize) = (risks.len() - 1, risks[0].len() - 1);
    let result = dijkstra(
        &(0, 0),
        |&(x, y)| {
            let mut v: SmallVec<[((usize, usize), u32); 4]> = smallvec![];
            if x < goal.0 {
                v.push(((x + 1, y), risks[x + 1][y]));
            }
            if y < goal.1 {
                v.push(((x, y + 1), risks[x][y + 1]));
            }
            if x > 0 {
                v.push(((x - 1, y), risks[x - 1][y]));
            }
            if y > 0 {
                v.push(((x, y - 1), risks[x][y - 1]));
            }
            v
        },
        |&p| p == goal,
    )
    .unwrap();
    result.1
}
