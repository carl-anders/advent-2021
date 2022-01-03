use super::day::Day;

#[derive(Debug, Clone, Copy)]
pub struct Area {
    x: [i32; 2],
    y: [i32; 2],
}
pub struct Day17;
impl Day for Day17 {
    type Parsed = Area;

    fn parse(mut input: impl Iterator<Item = String>) -> Self::Parsed {
        let vals: Vec<i32> = input
            .next()
            .unwrap()
            .split(&['=', '.', ','][..])
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        Area {
            x: [vals[0], vals[1]],
            y: [vals[2], vals[3]],
        }
    }
    fn first(area: Area) -> String {
        let max_y = area.y[0].abs().max(area.y[1].abs()) + 1;
        let max_x = area.x[0].abs().max(area.x[1].abs()) + 1;
        for init_y in (0..max_y).rev() {
            let mut y_top = 0;
            for init_x in 0..max_x {
                let (mut y, mut x, mut y_vel, mut x_vel) = (0, 0, init_y, init_x);
                while y >= area.y[0] && x <= area.x[1] {
                    y += y_vel;
                    y_top = y_top.max(y);
                    y_vel -= 1;
                    x += x_vel;
                    if x_vel > 0 {
                        x_vel -= 1;
                    }
                    if y >= area.y[0] && y <= area.y[1] && x >= area.x[0] && x <= area.x[1] {
                        return y_top.to_string();
                    }
                }
            }
        }
        panic!()
    }
    fn second(area: Area) -> String {
        let mut hits = 0;
        let max_y = area.y[0].abs().max(area.y[1].abs()) + 1;
        let max_x = area.x[0].abs().max(area.x[1].abs()) + 1;
        for init_y in -max_y..max_y {
            for init_x in 0..max_x {
                let (mut y, mut x, mut y_vel, mut x_vel) = (0, 0, init_y, init_x);
                while y >= area.y[0] && x <= area.x[1] {
                    y += y_vel;
                    y_vel -= 1;
                    x += x_vel;
                    if x_vel > 0 {
                        x_vel -= 1;
                    } else if !(x >= area.x[0] && x <= area.x[1]) {
                        break;
                    }
                    if y >= area.y[0] && y <= area.y[1] && x >= area.x[0] && x <= area.x[1] {
                        hits += 1;
                        break;
                    }
                }
            }
        }
        hits.to_string()
    }
}
