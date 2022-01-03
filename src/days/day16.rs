use super::day::Day;
use bitvec::prelude::*;

pub struct Day16;
impl Day for Day16 {
    type Parsed = Vec<Vec<u8>>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input
            .map(|s| {
                (0..s.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
                    .collect()
            })
            .collect()
    }
    fn first(list: Self::Parsed) -> String {
        let mut versions = Vec::new();
        for data in list {
            let bits = data.view_bits::<Msb0>();
            versions.push(parse(bits, &mut 0).0.to_string());
        }
        versions.join(", ")
    }
    fn second(list: Self::Parsed) -> String {
        let mut values = Vec::new();
        for data in list {
            let bits = data.view_bits::<Msb0>();
            values.push(parse(bits, &mut 0).1.to_string());
        }
        values.join(", ")
    }
}

fn parse(data: &BitSlice<Msb0, u8>, pos: &mut usize) -> (u16, u64) {
    let mut version = read(data, pos, 3);
    let type_id = read(data, pos, 3);
    if type_id == 4 {
        let mut value = 0;
        loop {
            let more = read(data, pos, 1);
            let data = read(data, pos, 4);
            value = (value << 4) + (data as u64);
            if more == 0 {
                return (version, value);
            }
        }
    }
    let length_type = read(data, pos, 1) == 1;
    let mut length = read(data, pos, if length_type { 11 } else { 15 });
    let mut start_bits = *pos;
    let mut value = None;
    while length > 0 {
        let (ve, va) = parse(data, pos);
        version += ve;
        value = Some(match type_id {
            0 => value.unwrap_or(0) + va,
            1 => value.unwrap_or(1) * va,
            2 => value.map_or(va, |v| v.min(va)),
            3 => value.map_or(va, |v| v.max(va)),
            5 => value.map_or(va, |v| (v > va).into()),
            6 => value.map_or(va, |v| (v < va).into()),
            7 => value.map_or(va, |v| (v == va).into()),
            _ => unreachable!(),
        });
        if length_type {
            length -= 1;
        } else {
            length -= (*pos - start_bits) as u16;
            start_bits = *pos;
        }
    }
    (version, value.unwrap())
}

fn read(data: &BitSlice<Msb0, u8>, pos: &mut usize, num: usize) -> u16 {
    *pos += num;
    data.get((*pos - num)..*pos)
        .unwrap()
        .iter()
        .fold(0, |s, b| (s << 1) + (*b as u16))
}
