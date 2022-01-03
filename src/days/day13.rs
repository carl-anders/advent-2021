use super::day::Day;

#[derive(Debug, Clone)]
pub struct Paper {
    fields: Vec<(i32, i32)>,
    instructions: Vec<(bool, i32)>,
}

pub struct Day13;
impl Day for Day13 {
    type Parsed = Paper;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        let mut output = Paper {
            fields: Vec::new(),
            instructions: Vec::new(),
        };
        let mut fields = true;
        for line in input {
            if line.is_empty() {
                fields = false;
                continue;
            }
            if fields {
                let vals = line.split_once(',').unwrap();
                output.fields.push((
                    vals.0.parse::<i32>().unwrap(),
                    vals.1.parse::<i32>().unwrap(),
                ));
            } else {
                let x = line.chars().nth(11) == Some('x');
                let val: i32 = line.split_once('=').unwrap().1.parse().unwrap();
                output.instructions.push((x, val));
            }
        }
        output
    }
    fn first(paper: Self::Parsed) -> String {
        let instructions = paper.instructions;
        let mut fields = paper.fields;
        fold_against(&mut fields, instructions[0]);
        fields.sort_unstable();
        fields.dedup();
        fields.len().to_string()
    }

    fn second(paper: Self::Parsed) -> String {
        let instructions = paper.instructions;
        let mut fields = paper.fields;
        for fold in instructions {
            fold_against(&mut fields, fold);
        }
        fields.sort_unstable();
        fields.dedup();
        let max_x = fields.iter().map(|i| i.0).max().unwrap();
        let max_y = fields.iter().map(|i| i.1).max().unwrap();
        let mut output = String::new();
        for y in 0..=max_y {
            output += "\n";
            for x in 0..=max_x {
                if fields.contains(&(x, y)) {
                    output += "█";
                } else {
                    output += " ";
                }
            }
        }
        output
    }
}

fn fold_against(fields: &mut Vec<(i32, i32)>, fold: (bool, i32)) {
    for field in fields {
        if fold.0 && field.0 > fold.1 {
            field.0 = fold.1 * 2 - field.0;
        } else if !fold.0 && field.1 > fold.1 {
            field.1 = fold.1 * 2 - field.1;
        }
    }
}
