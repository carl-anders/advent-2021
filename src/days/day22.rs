use super::day::Day;

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    on: bool,
    cube: Cube,
}

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    x: [i64; 2],
    y: [i64; 2],
    z: [i64; 2],
}
impl Cube {
    const fn intersects(&self, other: &Self) -> bool {
        self.x[1] >= other.x[0]
            && self.x[0] <= other.x[1]
            && self.y[1] >= other.y[0]
            && self.y[0] <= other.y[1]
            && self.z[1] >= other.z[0]
            && self.z[0] <= other.z[1]
    }
    fn get_outsides_z(&self, other: &Self) -> Vec<Self> {
        let mut outsides = Vec::new();
        if self.z[0] < other.z[0] {
            let mut left = *self;
            left.z[1] = other.z[0] - 1;
            outsides.push(left);
        }
        if self.z[1] > other.z[1] {
            let mut right = *self;
            right.z[0] = other.z[1] + 1;
            outsides.push(right);
        }
        let mut middle = *self;
        middle.z[0] = self.z[0].max(other.z[0]);
        middle.z[1] = self.z[1].min(other.z[1]);
        outsides.push(middle);
        outsides
    }
    fn get_outsides_yz(&self, other: &Self) -> Vec<Self> {
        let mut outsides = Vec::new();
        if self.y[0] < other.y[0] {
            let mut left = *self;
            left.y[1] = other.y[0] - 1;
            outsides.append(&mut left.get_outsides_z(other));
        }
        if self.y[1] > other.y[1] {
            let mut right = *self;
            right.y[0] = other.y[1] + 1;
            outsides.append(&mut right.get_outsides_z(other));
        }
        let mut middle = *self;
        middle.y[0] = self.y[0].max(other.y[0]);
        middle.y[1] = self.y[1].min(other.y[1]);
        outsides.append(&mut middle.get_outsides_z(other));
        outsides
    }
    fn get_outsides_xyz(&self, other: &Self) -> Vec<Self> {
        let mut outsides = Vec::new();
        if self.x[0] < other.x[0] {
            let mut left = *self;
            left.x[1] = other.x[0] - 1;
            outsides.append(&mut left.get_outsides_yz(other));
        }
        if self.x[1] > other.x[1] {
            let mut right = *self;
            right.x[0] = other.x[1] + 1;
            outsides.append(&mut right.get_outsides_yz(other));
        }
        let mut middle = *self;
        middle.x[0] = self.x[0].max(other.x[0]);
        middle.x[1] = self.x[1].min(other.x[1]);
        outsides.append(&mut middle.get_outsides_yz(other));

        // Remove cubes that are fully inside
        outsides.retain(|cube| !cube.inside(other));
        outsides
    }
    fn insert_from(self, cubes: &mut Vec<Self>) {
        for cube in cubes.iter() {
            if self.intersects(cube) {
                if !self.inside(cube) {
                    for ins in self.get_outsides_xyz(cube) {
                        ins.insert_from(cubes);
                    }
                }
                return;
            }
        }
        cubes.push(self);
    }
    fn remove_from(self, cubes: &mut Vec<Self>) {
        let mut i = 0;
        while i < cubes.len() {
            if cubes[i].intersects(&self) {
                if cubes[i].inside(&self) {
                    cubes.remove(i);
                } else {
                    let cube = cubes.remove(i);
                    cubes.append(&mut cube.get_outsides_xyz(&self));
                }
            } else {
                i += 1;
            }
        }
    }
    const fn inside(&self, container: &Self) -> bool {
        !(self.x[0] < container.x[0]
            || self.x[1] > container.x[1]
            || self.y[0] < container.y[0]
            || self.y[1] > container.y[1]
            || self.z[0] < container.z[0]
            || self.z[1] > container.z[1])
    }
    const fn volume(&self) -> i64 {
        (self.x[1] - self.x[0] + 1) * (self.y[1] - self.y[0] + 1) * (self.z[1] - self.z[0] + 1)
    }
}

pub struct Day22;
impl Day for Day22 {
    type Parsed = Vec<Instruction>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input
            .map(|l| {
                let on = l.chars().nth(1).unwrap() == 'n';
                let mut parts = l
                    .split(&['=', ',', '.'][..])
                    .filter_map(|s| s.parse::<i64>().ok());
                Instruction {
                    on,
                    cube: Cube {
                        x: [parts.next().unwrap(), parts.next().unwrap()],
                        y: [parts.next().unwrap(), parts.next().unwrap()],
                        z: [parts.next().unwrap(), parts.next().unwrap()],
                    },
                }
            })
            .collect()
    }

    fn first(instructions: Self::Parsed) -> String {
        let area = Cube {
            x: [-50, 50],
            y: [-50, 50],
            z: [-50, 50],
        };
        let mut cubes = Vec::new();
        for inst in instructions {
            if inst.cube.inside(&area) {
                if inst.on {
                    inst.cube.insert_from(&mut cubes);
                } else {
                    inst.cube.remove_from(&mut cubes);
                }
            }
        }
        cubes.iter().map(Cube::volume).sum::<i64>().to_string()
    }

    fn second(instructions: Self::Parsed) -> String {
        let mut cubes = Vec::new();
        for inst in instructions {
            if inst.on {
                inst.cube.insert_from(&mut cubes);
            } else {
                inst.cube.remove_from(&mut cubes);
            }
        }
        cubes.iter().map(Cube::volume).sum::<i64>().to_string()
    }
}
