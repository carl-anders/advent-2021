use super::day::Day;

const fn get_num(segments: [bool; 7]) -> u8 {
    match segments {
        [true, true, true, false, true, true, true] => 0, // 6 segments
        [false, false, true, false, false, true, false] => 1, // 2 segments
        [true, false, true, true, true, false, true] => 2, // 5 segments
        [true, false, true, true, false, true, true] => 3, // 5 segments
        [false, true, true, true, false, true, false] => 4, // 4 segments
        [true, true, false, true, false, true, true] => 5, // 5 segments
        [true, true, false, true, true, true, true] => 6, // 6 segments
        [true, false, true, false, false, true, false] => 7, // 3 segments
        [true, true, true, true, true, true, true] => 8,  // 7 segments
        [true, true, true, true, false, true, true] => 9, // 6 segments
        _ => 99,
    }
}

fn unique_char(from: &str, filter: &str) -> char {
    from.chars().find(|c| !filter.contains(*c)).unwrap()
}

pub struct Day8;
impl Day for Day8 {
    type Parsed = Vec<String>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input.collect()
    }

    fn first(lines: Vec<String>) -> String {
        let num_selected_digits: usize = lines
            .iter()
            .map(|line| {
                line.split_once(" | ")
                    .unwrap()
                    .1
                    .split(' ')
                    .filter(|disp| matches!(disp.len(), 2 | 3 | 4 | 7))
                    .count()
            })
            .sum();
        num_selected_digits.to_string()
    }
    fn second(lines: Vec<String>) -> String {
        let mut total_numbers = 0;
        for line in lines {
            let (observations, nums) = line.split_once(" | ").unwrap();
            let observations: Vec<&str> = observations.split(' ').collect();
            let nums: Vec<&str> = nums.split(' ').collect();

            let mut numbers = [""; 10];
            numbers[1] = observations.iter().find(|o| o.len() == 2).unwrap();
            numbers[4] = observations.iter().find(|o| o.len() == 4).unwrap();
            numbers[7] = observations.iter().find(|o| o.len() == 3).unwrap();
            numbers[8] = observations.iter().find(|o| o.len() == 7).unwrap();

            let mut chars = ['_'; 7];
            chars[0] = unique_char(numbers[7], numbers[1]);

            let unique_for_069: String = observations
                .iter()
                .filter(|o| o.len() == 6)
                .map(|o| unique_char(numbers[8], o))
                .collect();
            chars[5] = unique_char(numbers[1], &unique_for_069);
            chars[2] = unique_char(numbers[1], &chars[5].to_string());
            chars[1] = unique_char(numbers[4], &format!("{}{}", numbers[1], unique_for_069));
            chars[3] = unique_char(numbers[4], &format!("{}{}", numbers[1], chars[1]));
            chars[6] = unique_char(
                numbers[8],
                &format!("{}{}{}{}", unique_for_069, chars[0], chars[1], chars[5]),
            );
            chars[4] = unique_char(numbers[8], &chars.iter().collect::<String>());

            total_numbers += nums
                .iter()
                .map(|num| {
                    let mut lighted = [false; 7];
                    for c in num.chars() {
                        lighted[chars.iter().position(|&x| x == c).unwrap()] = true;
                    }
                    get_num(lighted) as u32
                })
                .fold(0, |acc, b| acc * 10 + b);
        }
        total_numbers.to_string()
    }
}
