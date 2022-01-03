use super::day::Day;

pub struct Day3;
impl Day for Day3 {
    type Parsed = Vec<String>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input.collect()
    }
    fn first(lines: Vec<String>) -> String {
        let mut values = Vec::new();
        for row in lines {
            for (i, c) in row.chars().enumerate() {
                if values.len() <= i {
                    values.push(Vec::new());
                }
                values[i].push(c != '0');
            }
        }

        let gamma = values
            .iter()
            .map(|c| c.iter().filter(|x| **x).count() >= c.iter().filter(|x| !(**x)).count())
            .fold(0, |acc, b| acc * 2 + b as u32);

        let epsilon = values
            .iter()
            .map(|c| c.iter().filter(|x| **x).count() <= c.iter().filter(|x| !(**x)).count())
            .fold(0, |acc, b| acc * 2 + b as u32);

        (gamma * epsilon).to_string()
    }
    fn second(lines: Vec<String>) -> String {
        let mut oxygen = lines.clone();
        for i in 0..lines[0].len() {
            if oxygen.len() > 1 {
                let num_zeros = oxygen
                    .iter()
                    .filter(|l| (**l).chars().nth(i) == Some('0'))
                    .count();
                let num_ones = oxygen.len() - num_zeros;
                let oxygen_keep = if num_ones >= num_zeros { '1' } else { '0' };
                oxygen = oxygen
                    .into_iter()
                    .filter(|l| (**l).chars().nth(i).unwrap() == oxygen_keep)
                    .collect();
            }
        }
        let oxygen = oxygen[0]
            .chars()
            .fold(0, |acc, b| acc * 2 + b.to_digit(10).unwrap());

        let mut co2 = lines.clone();
        for i in 0..lines[0].len() {
            if co2.len() > 1 {
                let num_zeros = co2
                    .iter()
                    .filter(|l| (**l).chars().nth(i) == Some('0'))
                    .count();
                let num_ones = co2.len() - num_zeros;
                let co2_keep = if num_ones < num_zeros { '1' } else { '0' };
                co2 = co2
                    .into_iter()
                    .filter(|l| (*l).chars().nth(i).unwrap() == co2_keep)
                    .collect();
            }
        }
        let co2 = co2[0]
            .chars()
            .fold(0, |acc, b| acc * 2 + b.to_digit(10).unwrap());

        (oxygen * co2).to_string()
    }
}
