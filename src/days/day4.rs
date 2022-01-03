use super::day::Day;
use itertools::Itertools;

pub struct Day4;
impl Day for Day4 {
    type Parsed = (String, Vec<BingoBoard>);

    fn parse(mut lines: impl Iterator<Item = String>) -> Self::Parsed {
        let commands = lines.next().unwrap();
        let mut boards = Vec::new();
        for mut chunk in &lines.chunks(6) {
            chunk.next();
            boards.push(BingoBoard::new(
                &chunk
                    .map(|line| {
                        line.split(' ')
                            .filter_map(|value| value.parse::<i32>().ok())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            ));
        }

        (commands, boards)
    }

    fn first(input: Self::Parsed) -> String {
        let (commands, mut boards) = input;
        for command in commands.split(',') {
            if let Ok(command) = command.parse::<i32>() {
                for board in &mut boards {
                    board.cross(command);
                    if board.check_if_won() {
                        return board.get_score(command).to_string();
                    }
                }
            }
        }
        unreachable!()
    }

    fn second(input: Self::Parsed) -> String {
        let (commands, mut boards) = input;
        for command in commands.split(',') {
            if let Ok(command) = command.parse::<i32>() {
                for board in &mut boards {
                    board.cross(command);
                }
                if boards.len() == 1 {
                    if boards[0].check_if_won() {
                        return boards[0].get_score(command).to_string();
                    }
                } else {
                    boards.retain(|&board| !board.check_if_won());
                }
            }
        }
        unreachable!();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BingoBoard {
    board: [[(i32, bool); 5]; 5],
}
impl BingoBoard {
    fn new(nums: &[Vec<i32>]) -> Self {
        let mut bb = Self {
            board: [[(0, false); 5]; 5],
        };
        for (row, row_vec) in nums.iter().enumerate() {
            for (col, val) in row_vec.iter().enumerate() {
                bb.board[row][col].0 = *val;
            }
        }
        bb
    }
    fn cross(&mut self, num: i32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col].0 == num {
                    self.board[row][col].1 = true;
                }
            }
        }
    }
    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col].1 {
                    print!(" X ");
                } else {
                    print!("{:>2} ", self.board[row][col].0);
                }
            }
            println!();
        }
    }
    fn check_if_won(&self) -> bool {
        for row in 0..5 {
            if self.board[row].iter().filter(|i| i.1).count() == 5 {
                return true;
            }
        }
        for col in 0..5 {
            if self.board.iter().map(|r| r[col]).filter(|i| i.1).count() == 5 {
                return true;
            }
        }
        false
    }
    fn points(&self) -> i32 {
        self.board.iter().fold(0, |acc, x| {
            acc + x
                .iter()
                .fold(0, |acc2, x2| if x2.1 { acc2 } else { acc2 + x2.0 })
        })
    }
    fn get_score(&self, command: i32) -> i32 {
        command * self.points()
    }
}
