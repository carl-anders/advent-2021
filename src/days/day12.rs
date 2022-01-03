use super::common::BitArr;
use super::day::Day;
use std::collections::{hash_map::Entry, HashMap};

fn is_big(name: &str) -> bool {
    name.chars().next().unwrap().is_ascii_uppercase()
}

#[derive(Clone)]
pub struct Exploration {
    connections: Vec<(bool, Vec<usize>)>,
}
impl Exploration {
    const START: usize = 0;
    const END: usize = 1;
    fn explore(&self, current: usize, mut visited: usize, twice: bool) -> i32 {
        if current == Self::END {
            1
        } else {
            let mut explorations = 0;
            unsafe {
                for &next in &self.connections.get_unchecked(current).1 {
                    if self.connections.get_unchecked(next).0 || !visited.get(next) {
                        visited.set(next);
                        explorations += self.explore(next, visited, twice);
                        visited.clear(next);
                    } else if next != Self::START && !twice {
                        explorations += self.explore(next, visited, true);
                    }
                }
            }
            explorations
        }
    }
}

pub struct Day12;
impl Day for Day12 {
    type Parsed = Exploration;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        let mut names = HashMap::new();
        let mut connections = Vec::new();
        names.insert("start".to_string(), Exploration::START);
        connections.push((false, vec![]));
        names.insert("end".to_string(), Exploration::END);
        connections.push((false, vec![]));

        for line in input {
            let (from, to) = line.split_once('-').unwrap();

            if let Entry::Vacant(e) = names.entry(from.to_string()) {
                e.insert(connections.len());
                connections.push((is_big(from), vec![]));
            }
            if let Entry::Vacant(e) = names.entry(to.to_string()) {
                e.insert(connections.len());
                connections.push((is_big(to), vec![]));
            }

            let from_num = names[from];
            let to_num = names[to];
            connections[from_num].1.push(to_num);
            connections[to_num].1.push(from_num);
        }

        Exploration { connections }
    }
    fn first(exploration: Self::Parsed) -> String {
        exploration.explore(0, 1, true).to_string()
    }
    fn second(exploration: Self::Parsed) -> String {
        exploration.explore(0, 1, false).to_string()
    }
}
/*fn explore(connections: &HashMap<String, Vec<String>>, visited: Vec<String>) -> Vec<Vec<String>> {
    let position = visited.last().unwrap();
    //println!("Visiting with path: {}", visited.join(" -> "));

    let mut explorations = Vec::new();
    if position == "end" {
        explorations.push(visited);
    } else {
        for next in connections.get(position).unwrap() {
            if is_big(next) || !visited.contains(next) {
                let mut new_visited = visited.clone();
                new_visited.push(next.clone());
                explorations.append(&mut explore(connections, new_visited));
            }
        }
    }
    explorations
}
fn explore_more(
    connections: &HashMap<String, Vec<String>>,
    visited: &mut Vec<String>,
    has_spelunked: bool,
) -> Vec<Vec<String>> {
    let position = visited.last().unwrap();
    //println!("Visiting with path: {}", visited.join(" -> "));

    let mut explorations = Vec::new();
    if position == "end" {
        explorations.push(visited.clone());
    } else {
        for next in connections.get(position).unwrap() {
            if is_big(next) || !visited.contains(next) {
                visited.push(next.clone());
                explorations.append(&mut explore_more(connections, visited, has_spelunked));
                visited.pop();
            } else if next != "start"
                && !has_spelunked
                && visited.iter().filter(|&v| v == next).count() < 2
            {
                visited.push(next.clone());
                explorations.append(&mut explore_more(connections, visited, true));
                visited.pop();
            }
        }
    }
    explorations
}
fn explore_less(
    connections: &HashMap<String, Vec<String>>,
    visited: &mut Vec<String>,
    has_spelunked: bool,
) -> i32 {
    let position = visited.last().unwrap();

    let mut explorations = 0;
    if position == "end" {
        explorations += 1;
    } else {
        for next in connections.get(position).unwrap() {
            if is_big(next) || !visited.contains(next) {
                visited.push(next.clone());
                explorations += explore_less(connections, visited, has_spelunked);
                visited.pop();
            } else if next != "start"
                && !has_spelunked
                && visited.iter().filter(|&v| v == next).count() < 2
            {
                visited.push(next.clone());
                explorations += explore_less(connections, visited, true);
                visited.pop();
            }
        }
    }
    explorations
}

pub struct Day12;
impl Day for Day12 {
    type Parsed = HashMap<String, Vec<String>>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        let mut connections = HashMap::new();
        for line in input {
            let (from, to) = line.split_once('-').unwrap();
            connections
                .entry(from.to_string())
                .or_insert_with(Vec::new)
                .push(to.to_string());
            connections
                .entry(to.to_string())
                .or_insert_with(Vec::new)
                .push(from.to_string());
        }
        connections
    }
    fn first(connections: Self::Parsed) -> String {
        let explorations = explore_less(&connections, &mut vec!["start".to_string()], true);
        /*for exploration in &explorations {
            println!("Working path: {}", exploration.join(" -> "));
        }*/
        explorations.to_string()
    }
    fn second(connections: Self::Parsed) -> String {
        let explorations = explore_less(&connections, &mut vec!["start".to_string()], false);
        /*for exploration in &explorations {
            println!("Working path: {}", exploration.join(" -> "));
        }*/
        explorations.to_string()
    }
}
*/
