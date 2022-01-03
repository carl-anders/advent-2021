use super::day::Day;

fn flash(octi: &mut Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    match octi.get(i).and_then(|c| c.get(j)) {
        Some(n) if n >= &9 => {
            octi[i][j] = 0;
            1 + flash(octi, i.wrapping_sub(1), j)
                + flash(octi, i.wrapping_sub(1), j + 1)
                + flash(octi, i, j + 1)
                + flash(octi, i + 1, j + 1)
                + flash(octi, i + 1, j)
                + flash(octi, i + 1, j.wrapping_sub(1))
                + flash(octi, i, j.wrapping_sub(1))
                + flash(octi, i.wrapping_sub(1), j.wrapping_sub(1))
        }
        Some(n) if n > &0 => {
            octi[i][j] += 1;
            0
        }
        _ => 0,
    }
}
fn advance(octi: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashes = 0;
    for i in octi.iter_mut() {
        for j in i {
            *j += 1;
        }
    }
    for i in 0..octi.len() {
        for j in 0..octi[i].len() {
            if octi[i][j] == 10 {
                flashes += flash(octi, i, j);
            }
        }
    }
    flashes
}
#[allow(dead_code)]
fn print(octi: &[Vec<u32>]) {
    for i in octi {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}

pub struct Day11;
impl Day for Day11 {
    type Parsed = Vec<Vec<u32>>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input
            .map(|line| line.chars().filter_map(|n| n.to_digit(10)).collect())
            .collect()
    }
    fn first(mut octi: Vec<Vec<u32>>) -> String {
        let mut flashes = 0;
        for _ in 0..100 {
            flashes += advance(&mut octi);
        }
        flashes.to_string()
    }
    fn second(mut octi: Vec<Vec<u32>>) -> String {
        let mut iteration = 0;
        loop {
            iteration += 1;
            advance(&mut octi);
            if octi.iter().map(|v| v.iter().sum::<u32>()).sum::<u32>() == 0 {
                break;
            }
        }
        iteration.to_string()
    }
}
