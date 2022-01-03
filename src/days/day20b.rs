use super::day::Day;

#[derive(Clone, Debug)]
pub struct Image {
    table: Vec<bool>,
    data: Vec<Vec<bool>>,
}
impl Image {
    pub fn grow(&mut self, by: usize) {
        let side_filler = vec![false; by];
        for v in &mut self.data {
            let mut filled = side_filler.clone();
            filled.append(v);
            filled.append(&mut side_filler.clone());
            *v = filled;
        }
        let mut filler = vec![vec![false; self.data[0].len()]; by];
        let mut filled = filler.clone();
        filled.append(&mut self.data);
        filled.append(&mut filler);
        self.data = filled;
    }
    pub fn shrink(&mut self, by: usize) {
        let len = self.data.len();
        self.data = self
            .data
            .clone()
            .into_iter()
            .skip(by)
            .take(len - by * 2)
            .map(|v| v.into_iter().skip(by).take(len - by * 2).collect())
            .collect();
    }
    pub fn enhance(&mut self) {
        let mut new_data = self.data.clone();
        for row in 1..(self.data.len() - 1) {
            for col in 1..(self.data[1].len() - 1) {
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
                            + *self
                                .data
                                .get_unchecked((row as i64 + ar) as usize)
                                .get_unchecked((col as i64 + ac) as usize)
                                as usize;
                    }
                    *new_data.get_unchecked_mut(row).get_unchecked_mut(col) =
                        *self.table.get_unchecked(num);
                }
            }
        }
        self.data = new_data;
    }
    #[allow(dead_code)]
    pub fn show(&self) {
        for row in &self.data {
            for col in row {
                print!("{}", if *col { '█' } else { ' ' });
            }
            println!();
        }
    }
    pub fn num_lit(&self) -> i64 {
        self.data
            .iter()
            .map(|r| r.iter().filter(|&&b| b).count() as i64)
            .sum()
    }
}
pub struct Day20;
impl Day for Day20 {
    type Parsed = Image;

    fn parse(mut input: impl Iterator<Item = String>) -> Self::Parsed {
        let table = input.next().unwrap().chars().map(|c| c == '#').collect();
        let data: Vec<Vec<bool>> = input
            .skip(1)
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .collect();
        Image { table, data }
    }

    fn first(mut image: Self::Parsed) -> String {
        image.grow(4);
        for _ in 0..2 {
            image.enhance();
        }
        image.shrink(2);

        image.num_lit().to_string()
    }

    fn second(mut image: Self::Parsed) -> String {
        image.grow(100);
        for _ in 0..50 {
            image.enhance();
        }
        image.shrink(50);

        image.num_lit().to_string()
    }
}
