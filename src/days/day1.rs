use super::day::Day;

pub struct Day1;
impl Day for Day1 {
    type Parsed = Vec<i32>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input.filter_map(|s| s.parse::<i32>().ok()).collect()
    }
    fn first(numbers: Vec<i32>) -> String {
        let mut last_num = i32::MAX;
        let mut num_increases = 0;

        for num in numbers {
            if num > last_num {
                num_increases += 1;
            }
            last_num = num;
        }

        num_increases.to_string()
    }
    fn second(numbers: Vec<i32>) -> String {
        let mut combined_nums = Vec::new();

        for (line_num, num) in numbers.iter().enumerate() {
            if line_num < numbers.len() - 2 {
                combined_nums.push(num + numbers[line_num + 1] + numbers[line_num + 2]);
            }
        }

        Self::first(combined_nums)
    }
}
