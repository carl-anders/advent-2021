use super::day::Day;

pub struct Day2;
impl Day for Day2 {
    type Parsed = Vec<String>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input.collect()
    }
    fn first(paths: Vec<String>) -> String {
        let mut horizontal = 0;
        let mut depth = 0;
        for path in paths {
            if let Some((command, value)) = path.split_once(' ') {
                let value = value.parse::<i32>().unwrap();
                match command {
                    "forward" => horizontal += value,
                    "down" => depth += value,
                    "up" => depth -= value,
                    _ => {}
                }
            }
        }
        (horizontal * depth).to_string()
    }
    fn second(paths: Vec<String>) -> String {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;
        for path in paths {
            if let Some((command, value)) = path.split_once(' ') {
                let value = value.parse::<i32>().unwrap();
                match command {
                    "forward" => {
                        horizontal += value;
                        depth += aim * value;
                    }
                    "down" => aim += value,
                    "up" => aim -= value,
                    _ => {}
                }
            }
        }
        (horizontal * depth).to_string()
    }
}
