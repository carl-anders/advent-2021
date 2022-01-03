use super::day::Day;

fn simulate(days: i32, fish: Vec<i8>) -> u64 {
    let mut map = [0; 9];
    for fish in fish {
        map[fish as usize] += 1;
    }
    for _ in 1..=days {
        let spawners = map[0];
        for i in 0..map.len() - 1 {
            map[i] = map[i + 1];
        }
        map[6] += spawners;
        map[8] = spawners;
    }
    map.iter().sum()
}

pub struct Day6;
impl Day for Day6 {
    type Parsed = Vec<i8>;

    fn parse(mut input: impl Iterator<Item = String>) -> Self::Parsed {
        input
            .next()
            .unwrap()
            .split(',')
            .map(|i| i.parse::<i8>().ok().unwrap())
            .collect()
    }
    fn first(fish: Vec<i8>) -> String {
        simulate(80, fish).to_string()
    }
    fn second(fish: Vec<i8>) -> String {
        simulate(256, fish).to_string()
    }
}
