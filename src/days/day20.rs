use super::day::Day;

#[derive(Clone, Debug)]
pub struct Image {
    table: Vec<bool>,
    data: Vec<bool>,
    width: usize,
    height: usize,
    inf: bool,
}
impl Image {
    #[allow(dead_code)]
    pub fn show(&self) {
        let mut x_pos = 0;
        for row in &self.data {
            print!("{}", if *row { '█' } else { ' ' });
            x_pos += 1;
            if x_pos == self.width {
                x_pos = 0;
                println!();
            }
        }
        println!();
    }
    pub fn checked_set(&mut self, new_data: &mut Vec<bool>, row: usize, col: usize) {
        let mut num = 0;
        for (ar, ac) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let nr = (row as i32) + ar - 1;
            let nc = (col as i32) + ac - 1;
            if 0 <= nr && nr < self.width as i32 && 0 <= nc && nc < self.height as i32 {
                num = (num << 1) + self.data[(nr * self.width as i32 + nc) as usize] as usize;
            } else {
                num = (num << 1) + self.inf as usize;
            }
        }
        new_data[row * (self.width + 2) + col] = self.table[num];
    }
    pub fn enhance(&mut self) {
        let (width, height) = (self.width + 2, self.height + 2);
        let mut new_data = vec![false; width * height];
        if width < 5 || height < 5 {
            for row in 0..height {
                for col in 0..width {
                    self.checked_set(&mut new_data, row, col);
                }
            }
        } else {
            for row in 0..2 {
                for col in 0..width {
                    self.checked_set(&mut new_data, row, col);
                }
            }
            for row in 2..(height - 2) {
                self.checked_set(&mut new_data, row, 0);
                self.checked_set(&mut new_data, row, 1);
                let rp = row * (self.width + 2);
                for col in 2..(width - 2) {
                    unsafe {
                        let mut num = 0;
                        for (ar, ac) in [
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, -1),
                            (0, 0),
                            (0, 1),
                            (1, -1),
                            (1, 0),
                            (1, 1),
                        ] {
                            num = (num << 1)
                                | *self.data.get_unchecked(
                                    (((row as i32) + ar - 1) * self.width as i32
                                        + ((col as i32) + ac - 1))
                                        as usize,
                                ) as usize;
                        }
                        *new_data.get_unchecked_mut(rp + col) = *self.table.get_unchecked(num);
                    }
                }
                self.checked_set(&mut new_data, row, width - 2);
                self.checked_set(&mut new_data, row, width - 1);
            }
            for row in (height - 2)..height {
                for col in 0..width {
                    self.checked_set(&mut new_data, row, col);
                }
            }
        }
        self.data = new_data;
        self.width = width;
        self.height = height;
        self.inf = self.table[if self.inf { 511 } else { 0 }];
    }
    pub fn num_lit(&self) -> i64 {
        self.data.iter().filter(|&&b| b).count() as i64
    }
}
pub struct Day20;
impl Day for Day20 {
    type Parsed = Image;

    fn parse(mut input: impl Iterator<Item = String>) -> Self::Parsed {
        let table = input.next().unwrap().chars().map(|c| c == '#').collect();
        let mut width = 0;
        let data: Vec<bool> = input
            .skip(1)
            .flat_map(|l| {
                if width == 0 {
                    width = l.chars().count();
                }
                l.chars().map(|c| c == '#').collect::<Vec<bool>>()
            })
            .collect();
        Image {
            table,
            width,
            height: data.len() / width,
            data,
            inf: false,
        }
    }

    fn first(mut image: Self::Parsed) -> String {
        for _ in 0..2 {
            image.enhance();
        }

        image.num_lit().to_string()
    }

    fn second(mut image: Self::Parsed) -> String {
        for _ in 0..50 {
            image.enhance();
        }

        image.num_lit().to_string()
    }
}
