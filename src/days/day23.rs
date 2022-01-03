#![allow(
    clippy::ptr_arg,
    clippy::too_many_arguments,
    clippy::needless_pass_by_value
)]

use std::hash::Hash;

use pathfinding::directed::dijkstra::dijkstra;

use super::common::BitArr;
use super::day::Day;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum AmphipodKind {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Amphipod {
    kind: AmphipodKind,
    moved: bool,
}
impl Amphipod {
    const fn cost(self) -> i64 {
        match self.kind {
            AmphipodKind::Amber => 1,
            AmphipodKind::Bronze => 10,
            AmphipodKind::Copper => 100,
            AmphipodKind::Desert => 1000,
        }
    }
    const fn print(self) -> char {
        match self.kind {
            AmphipodKind::Amber => 'A',
            AmphipodKind::Bronze => 'B',
            AmphipodKind::Copper => 'C',
            AmphipodKind::Desert => 'D',
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum SlotKind {
    Corridor,
    Above,
    Nest(AmphipodKind),
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Slot {
    kind: SlotKind,
    links: Vec<usize>,
}
pub type Amphipods = Vec<Option<Amphipod>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Map {
    slots: Vec<Slot>,
}
impl Map {
    fn find_parking_for(
        &self,
        pods: &Amphipods,
        from: usize,
        curr: usize,
        mut visited: usize,
        cost: i64,
        depth: i64,
        output: &mut Vec<(Amphipods, i64)>,
    ) {
        visited.set(curr);
        for &link in &self.slots[curr].links {
            if !visited.get(link) && pods[link].is_none() {
                if self.slots[link].kind == SlotKind::Corridor {
                    let mut new_pods = pods.clone();
                    new_pods[link] = new_pods[from].take();
                    new_pods[link].unwrap().moved = true;
                    output.push((new_pods, cost * depth));
                }
                self.find_parking_for(pods, from, link, visited, cost, depth + 1, output);
            }
        }
    }
    fn check_if_deeper_nests_valid(
        &self,
        pods: &Amphipods,
        kind: AmphipodKind,
        curr: usize,
    ) -> bool {
        for &link in &self.slots[curr].links {
            if link > curr {
                if let Some(a) = pods[link] {
                    if a.kind != kind {
                        return false;
                    }
                }
                return self.check_if_deeper_nests_valid(pods, kind, link);
            }
        }
        true
    }
    fn find_nest_for(
        &self,
        pods: &Amphipods,
        from: usize,
        curr: usize,
        mut visited: usize,
        cost: i64,
        depth: i64,
        output: &mut Vec<(Amphipods, i64)>,
    ) -> bool {
        visited.set(curr);
        for &link in &self.slots[curr].links {
            if !visited.get(link) && pods[link].is_none() {
                if let SlotKind::Nest(kind) = self.slots[link].kind {
                    if let Some(a) = pods[from] {
                        if kind == a.kind {
                            if self.find_nest_for(
                                pods,
                                from,
                                link,
                                visited,
                                cost,
                                depth + 1,
                                output,
                            ) {
                                return true;
                            } else if self.check_if_deeper_nests_valid(pods, kind, link) {
                                let mut new_pods = pods.clone();
                                new_pods[link] = new_pods[from].take();
                                output.push((new_pods, cost * depth));
                                return true;
                            }
                            return false;
                        }
                    }
                } else if self.find_nest_for(pods, from, link, visited, cost, depth + 1, output) {
                    return true;
                }
            }
        }
        false
    }
    fn get_valid_maps(&self, pods: &Amphipods) -> Vec<(Amphipods, i64)> {
        let mut moves = Vec::new();
        for i in 0..self.slots.len() {
            if pods[i].is_some() {
                let cost = pods[i].unwrap().cost();
                match self.slots[i].kind {
                    SlotKind::Corridor => {
                        self.find_nest_for(pods, i, i, 0, cost, 1, &mut moves);
                    }
                    SlotKind::Above => {}
                    SlotKind::Nest(_) => {
                        if !pods[i].unwrap().moved {
                            self.find_parking_for(pods, i, i, 0, cost, 1, &mut moves);
                        }
                    }
                }
            }
        }
        moves
    }

    fn best_path(&self, pods: Amphipods) -> Option<i64> {
        dijkstra(
            &pods,
            |pods| self.get_valid_maps(pods),
            |current| {
                for (i, item) in current.iter().enumerate() {
                    if let SlotKind::Nest(kind) = self.slots[i].kind {
                        match item {
                            Some(amph) => {
                                if amph.kind != kind {
                                    return false;
                                }
                            }
                            None => {
                                return false;
                            }
                        }
                    }
                }
                true
            },
        )
        .map(|r| r.1)
    }

    #[allow(dead_code)]
    fn print(&self, pods: &Amphipods) {
        for pod in pods.iter().take(11) {
            print!(
                "{}",
                match pod {
                    Some(a) => a.print(),
                    None => '.',
                }
            );
        }
        for j in 0..((self.slots.len() - 11) / 4) {
            let add = j * 4 + 11;
            println!();
            print!(" ");
            for i in 0..4 {
                print!(
                    " {}",
                    match pods[add + i] {
                        Some(a) => a.print(),
                        None => '.',
                    }
                );
            }
        }
        println!("\n");
    }
}
pub struct Day23;
impl Day for Day23 {
    type Parsed = Vec<(Map, Amphipods)>;

    fn parse(inp: impl Iterator<Item = String>) -> Self::Parsed {
        let mut input: Vec<String> = inp.collect();

        let mut maps = Vec::new();

        for part in 0..2 {
            if part == 1 {
                input.insert(3, "  #D#C#B#A#".to_string());
                input.insert(4, "  #D#B#A#C#".to_string());
            }
            let mut aboves = Vec::new();
            let mut slots = Vec::new();
            let mut pods: Amphipods = Vec::new();
            for i in 0..input.len() {
                for j in 0..input[i].chars().count() {
                    match input[i].chars().nth(j).unwrap() {
                        '.' => {
                            let mut links = Vec::new();
                            let mut kind = SlotKind::Corridor;
                            if input[i].chars().nth(j - 1) != Some('#') {
                                links.push(j - 2);
                            }
                            if input[i].chars().nth(j + 1) != Some('#') {
                                links.push(j);
                            }
                            if input[i + 1].chars().nth(j) != Some('#') {
                                kind = SlotKind::Above;
                                aboves.push(j - 1);
                            }
                            slots.push(Slot { kind, links });
                            pods.push(None);
                        }
                        'A' | 'B' | 'C' | 'D' => {
                            let kind = match input[i].chars().nth(j).unwrap() {
                                'A' => AmphipodKind::Amber,
                                'B' => AmphipodKind::Bronze,
                                'C' => AmphipodKind::Copper,
                                'D' => AmphipodKind::Desert,
                                _ => panic!(),
                            };
                            let curr = slots.len();
                            let slot_kind = match aboves.len() % 4 {
                                0 => AmphipodKind::Amber,
                                1 => AmphipodKind::Bronze,
                                2 => AmphipodKind::Copper,
                                3 => AmphipodKind::Desert,
                                _ => panic!(),
                            };
                            let above = aboves[aboves.len() - 4];
                            slots[above].links.push(curr);
                            aboves.push(curr);

                            slots.push(Slot {
                                kind: SlotKind::Nest(slot_kind),
                                links: vec![above],
                            });
                            pods.push(Some(Amphipod { kind, moved: false }));
                        }
                        _ => {}
                    }
                }
            }
            maps.push((Map { slots }, pods));
        }

        maps
    }

    fn first(mut data: Self::Parsed) -> String {
        let first = data.remove(0);
        first.0.best_path(first.1).unwrap().to_string()
    }

    fn second(mut data: Self::Parsed) -> String {
        let second = data.remove(1);
        second.0.best_path(second.1).unwrap().to_string()
    }
}
