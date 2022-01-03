use ndarray::Array2;

use super::Day;

#[derive(Clone, Copy, Hash, PartialEq)]
enum SlotType {
    Right,
    Down,
    Empty,
}

#[derive(Clone, Hash, PartialEq)]
pub struct SeaCucumbers2 {
    slots: Array2<SlotType>,
    moved: Array2<bool>,
}
impl SeaCucumbers2 {
    fn new(slots: Array2<SlotType>) -> Self {
        Self {
            moved: Array2::default(slots.raw_dim()),
            slots,
        }
    }
    fn next(&mut self) -> bool {
        let mut any_moved = false;
        self.moved.fill(false);
        for i in 0..self.slots.dim().0 {
            for j in 0..self.slots.dim().1 {
                if self.slots[(i, j)] == SlotType::Right && !self.moved[(i, j)] {
                    let jp = (j + 1) % self.slots.dim().1;
                    if self.slots[(i, jp)] == SlotType::Empty && !self.moved[(i, jp)] {
                        self.slots[(i, jp)] = self.slots[(i, j)];
                        self.slots[(i, j)] = SlotType::Empty;
                        self.moved[(i, j)] = true;
                        self.moved[(i, jp)] = true;
                        any_moved = true;
                    }
                }
            }
        }
        self.moved.fill(false);
        for i in 0..self.slots.dim().0 {
            for j in 0..self.slots.dim().1 {
                if self.slots[(i, j)] == SlotType::Down && !self.moved[(i, j)] {
                    let ip = (i + 1) % self.slots.dim().0;
                    if self.slots[(ip, j)] == SlotType::Empty && !self.moved[(ip, j)] {
                        self.slots[(ip, j)] = self.slots[(i, j)];
                        self.slots[(i, j)] = SlotType::Empty;
                        self.moved[(i, j)] = true;
                        self.moved[(ip, j)] = true;
                        any_moved = true;
                    }
                }
            }
        }
        any_moved
    }
    #[allow(dead_code)]
    fn print(&self) {
        for v in self.slots.rows() {
            for slot in v {
                print!(
                    "{}",
                    match slot {
                        SlotType::Right => '>',
                        SlotType::Down => 'v',
                        SlotType::Empty => '.',
                    }
                );
            }
            println!();
        }
    }
}

pub struct Day25;
impl Day for Day25 {
    type Parsed = SeaCucumbers2;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        let mut slots = Vec::new();
        let mut rows = 0;
        let mut cols = 0;
        for line in input {
            rows += 1;
            if cols == 0 {
                cols = line.chars().count();
            } else if cols != line.chars().count() {
                panic!("Malformed 2d array");
            }
            for char in line.chars() {
                slots.push(match char {
                    '>' => SlotType::Right,
                    'v' => SlotType::Down,
                    _ => SlotType::Empty,
                });
            }
        }
        SeaCucumbers2::new(Array2::from_shape_vec((rows, cols), slots).unwrap())
        //SeaCucumbers::new(slots, (rows, cols))
    }

    fn first(mut sea_cucumbers: Self::Parsed) -> String {
        let mut steps = 0;
        while sea_cucumbers.next() {
            steps += 1;
        }

        (steps + 1).to_string()
    }

    fn second(_data: Self::Parsed) -> String {
        1.to_string()
    }
}

/*
#[derive(Clone, Hash, PartialEq)]
pub struct SeaCucumbers {
    slots: Vec<SlotType>,
    moved: Vec<bool>,
    rows: usize,
    cols: usize,
}
#[allow(dead_code)]
impl SeaCucumbers {
    fn new(slots: Vec<SlotType>, shape: (usize, usize)) -> Self {
        Self {
            moved: vec![false; slots.len()],
            slots,
            rows: shape.0,
            cols: shape.1,
        }
    }
    fn next(&mut self) -> bool {
        let mut any_moved = false;
        self.moved.fill(false);
        for i in 0..self.rows {
            for j in 0..self.cols {
                let row_pos = i * self.cols;
                let pos = row_pos + j;
                if self.slots[pos] == SlotType::Right && !self.moved[pos] {
                    let new_pos = row_pos + (j + 1) % self.cols;
                    if self.slots[new_pos] == SlotType::Empty && !self.moved[new_pos] {
                        self.slots[new_pos] = self.slots[pos];
                        self.slots[pos] = SlotType::Empty;
                        self.moved[pos] = true;
                        self.moved[new_pos] = true;
                        any_moved = true;
                    }
                }
            }
        }
        self.moved.fill(false);
        for i in 0..self.rows {
            for j in 0..self.cols {
                let pos = i * self.cols + j;
                if self.slots[pos] == SlotType::Down && !self.moved[pos] {
                    let new_pos = (pos + self.cols) % self.slots.len();
                    if self.slots[new_pos] == SlotType::Empty && !self.moved[new_pos] {
                        self.slots[new_pos] = self.slots[pos];
                        self.slots[pos] = SlotType::Empty;
                        self.moved[pos] = true;
                        self.moved[new_pos] = true;
                        any_moved = true;
                    }
                }
            }
        }
        any_moved
    }
    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!(
                    "{}",
                    match self.slots[i * self.cols + j] {
                        SlotType::Right => '>',
                        SlotType::Down => 'v',
                        SlotType::Empty => '.',
                    }
                );
            }
            println!();
        }
    }
}
*/
