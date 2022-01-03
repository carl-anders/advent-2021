use super::day::Day;

const fn distance(from: u32, to: u32) -> u32 {
    if from > to {
        from - to
    } else {
        to - from
    }
}
const fn triangular(num: u32) -> u64 {
    let num = num as u64;
    (num * (num + 1)) / 2
}
fn calculate<F>(crabs: &[u32], f: F) -> (u32, u64)
where
    F: Fn(u32, u32) -> u64,
{
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let mut best: Option<(u32, u64)> = None;
    for i in min..=max {
        let fuel: u64 = crabs.iter().map(|crab| f(i, *crab)).sum();
        match best {
            Some(f) if f.1 < fuel => {}
            _ => best = Some((i, fuel)),
        }
    }
    best.unwrap()
}

pub struct Day7;
impl Day for Day7 {
    type Parsed = Vec<u32>;

    fn parse(mut input: impl Iterator<Item = String>) -> Self::Parsed {
        input
            .next()
            .unwrap()
            .split(',')
            .map(|i| i.parse::<u32>().ok().unwrap())
            .collect()
    }
    fn first(crabs: Vec<u32>) -> String {
        let best = calculate(&crabs, |a, b| distance(a, b) as u64);
        best.1.to_string()
    }
    fn second(crabs: Vec<u32>) -> String {
        let best = calculate(&crabs, |a, b| triangular(distance(a, b)));
        best.1.to_string()
    }
}
