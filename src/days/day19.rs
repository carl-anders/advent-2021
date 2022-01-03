use itertools::iproduct;

use super::day::Day;

pub struct Day19;
impl Day for Day19 {
    type Parsed = Vec<Vec<[i64; 3]>>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        let mut data = Vec::new();
        for line in input {
            if line.starts_with("--") {
                data.push(Vec::new());
            } else if !line.is_empty() {
                let last = data.len() - 1;
                data[last].push(
                    line.split(',')
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                        .try_into()
                        .unwrap(),
                );
            }
        }
        for sensor in &mut data {
            sensor.sort_unstable();
        }
        data
    }

    fn first(data: Self::Parsed) -> String {
        let result = reduce_sensors(data);

        result[0].len().to_string()
    }

    fn second(data: Self::Parsed) -> String {
        let result = reduce_sensors(data);
        let sensors = &result[1];

        let max_distance = iproduct!(sensors.iter(), sensors.iter())
            .map(|(a, b)| abs_diff(a[0], b[0]) + abs_diff(a[1], b[1]) + abs_diff(a[2], b[2]))
            .max()
            .unwrap();

        max_distance.to_string()
    }
}

fn abs_diff<U: std::ops::Sub<Output = U> + std::cmp::PartialOrd>(a: U, b: U) -> U {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn reduce_sensors(mut data: Vec<Vec<[i64; 3]>>) -> [Vec<[i64; 3]>; 2] {
    println!(
        "Total sensors: {}. First sees: {}",
        data.len(),
        data[0].len()
    );
    let mut scanners = vec![[0, 0, 0]];
    while data.len() > 1 {
        scanners.push(reduce_pair(&mut data).unwrap());
        println!(
            "Total sensors: {}. First sees: {}",
            data.len(),
            data[0].len()
        );
    }
    [data.remove(0), scanners]
}

fn rotations() -> [fn([i64; 3]) -> [i64; 3]; 24] {
    [
        |n| [n[0], n[1], n[2]],
        |n| [n[0], -n[2], n[1]],
        |n| [n[0], -n[1], -n[2]],
        |n| [n[0], n[2], -n[1]],
        |n| [-n[1], n[0], n[2]],
        |n| [n[2], n[0], n[1]],
        |n| [n[1], n[0], -n[2]],
        |n| [-n[2], n[0], -n[1]],
        |n| [-n[0], -n[1], n[2]],
        |n| [-n[0], -n[2], -n[1]],
        |n| [-n[0], n[1], -n[2]],
        |n| [-n[0], n[2], n[1]],
        |n| [n[1], -n[0], n[2]],
        |n| [n[2], -n[0], -n[1]],
        |n| [-n[1], -n[0], -n[2]],
        |n| [-n[2], -n[0], n[1]],
        |n| [-n[2], n[1], n[0]],
        |n| [n[1], n[2], n[0]],
        |n| [n[2], -n[1], n[0]],
        |n| [-n[1], -n[2], n[0]],
        |n| [-n[2], -n[1], -n[0]],
        |n| [-n[1], n[2], -n[0]],
        |n| [n[2], n[1], -n[0]],
        |n| [n[1], -n[2], -n[0]],
    ]
}

fn reduce_pair(data: &mut Vec<Vec<[i64; 3]>>) -> Option<[i64; 3]> {
    let rotations = rotations();
    let outer = 0;
    for inner in 1..(data.len()) {
        let first = data[outer].clone();
        let second = data[inner].clone();

        for rot in rotations {
            for i in 0..(first.len()) {
                // Assuming first[i] as truth
                for j in 0..(second.len()) {
                    // Assume second[j] == first[i]
                    let truth = rot(second[j]);
                    let diff = [
                        first[i][0] - truth[0],
                        first[i][1] - truth[1],
                        first[i][2] - truth[2],
                    ];
                    // Check how many fits that truth
                    let mut num_fits = 0;
                    for k in &second {
                        let a = rot(*k);
                        let a = [a[0] + diff[0], a[1] + diff[1], a[2] + diff[2]];
                        if first.binary_search(&a).is_ok() {
                            num_fits += 1;
                        }
                    }
                    if num_fits > 10 {
                        for k in &second {
                            let a = rot(*k);
                            let a = [a[0] + diff[0], a[1] + diff[1], a[2] + diff[2]];
                            data[outer].push(a);
                        }
                        data[outer].sort_unstable();
                        data[outer].dedup();
                        data.remove(inner);
                        return Some(diff);
                    }
                }
            }
        }
    }
    None
}
