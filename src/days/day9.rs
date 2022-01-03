use super::day::Day;

fn spread(visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
    if visited.get(i).and_then(|c| c.get(j)) == Some(&false) {
        visited[i][j] = true;
        1 + spread(visited, i.wrapping_sub(1), j)
            + spread(visited, i, j + 1)
            + spread(visited, i + 1, j)
            + spread(visited, i, j.wrapping_sub(1))
    } else {
        0
    }
}

pub struct Day9;
impl Day for Day9 {
    type Parsed = Vec<Vec<u32>>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input
            .map(|line| line.chars().filter_map(|n| n.to_digit(10)).collect())
            .collect()
    }

    fn first(heightmap: Vec<Vec<u32>>) -> String {
        let mut risk_levels = 0;
        for i in 0..heightmap.len() {
            for j in 0..heightmap[i].len() {
                let val = heightmap[i][j];
                if val < 9
                    && !((i > 0 && heightmap[i - 1][j] < val)
                        || (j < heightmap[i].len() - 1 && heightmap[i][j + 1] < val)
                        || (i < heightmap.len() - 1 && heightmap[i + 1][j] < val)
                        || (j > 0 && heightmap[i][j - 1] < val))
                {
                    risk_levels += val + 1;
                }
            }
        }
        risk_levels.to_string()
    }
    fn second(heightmap: Vec<Vec<u32>>) -> String {
        let mut visited: Vec<Vec<bool>> = heightmap
            .iter()
            .map(|r| r.iter().map(|&n| n == 9).collect())
            .collect();
        let mut basins: Vec<u32> = Vec::new();
        for i in 0..visited.len() {
            for j in 0..visited[i].len() {
                if !visited[i][j] {
                    basins.push(spread(&mut visited, i, j));
                }
            }
        }
        basins.sort_unstable();
        basins.iter().rev().take(3).product::<u32>().to_string()
    }
}
/*
fn spread_t3(visited: &mut Vec<Vec<bool>>, i: i32, j: i32) -> u32 {
    if (0..(visited.len() as i32)).contains(&i)
        && (0..(visited[0].len() as i32)).contains(&j)
        && !visited[i as usize][j as usize]
    {
        visited[i as usize][j as usize] = true;
        1 + spread_t3(visited, i - 1, j)
            + spread_t3(visited, i, j + 1)
            + spread_t3(visited, i + 1, j)
            + spread_t3(visited, i, j - 1)
    } else {
        0
    }
}
fn spread_t2(visited: &mut Vec<Vec<bool>>, i: i32, j: i32) -> u32 {
    if i >= 0
        && (i as usize) < visited.len()
        && j >= 0
        && (j as usize) < visited[i as usize].len()
        && !visited[i as usize][j as usize]
    {
        visited[i as usize][j as usize] = true;
        1 + spread_t2(visited, i - 1, j)
            + spread_t2(visited, i, j + 1)
            + spread_t2(visited, i + 1, j)
            + spread_t2(visited, i, j - 1)
    } else {
        0
    }
}
fn spread_t1(visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
    visited[i][j] = true;
    let mut total = 1;
    if i > 0 && !visited[i - 1][j] {
        total += spread_t1(visited, i - 1, j);
    }
    if j < visited[i].len() - 1 && !visited[i][j + 1] {
        total += spread_t1(visited, i, j + 1);
    }
    if i < visited.len() - 1 && !visited[i + 1][j] {
        total += spread_t1(visited, i + 1, j);
    }
    if j > 0 && !visited[i][j - 1] {
        total += spread_t1(visited, i, j - 1);
    }
    total
}
*/
