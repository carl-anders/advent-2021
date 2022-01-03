#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::non_ascii_literal,
    clippy::match_on_vec_items,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::option_if_let_else,
    clippy::manual_assert
)]

use advent_2021_proc_macros::match_days;
use days::Day;
use itertools::Itertools;
use results::{DayResult, RESULTS};
use std::{
    fs::File,
    io::{self, BufRead},
    time::{Duration, Instant},
};
mod days;
mod results;

fn run<T: Day>(
    file: String,
    results: [&str; 2],
) -> (
    std::time::Duration,
    std::time::Duration,
    std::time::Duration,
) {
    let lines = File::open(file)
        .map(io::BufReader::new)
        .unwrap()
        .lines()
        .filter_map(Result::ok);

    let start_time = Instant::now();
    let parsed = T::parse(lines);
    let parsed_time = Instant::now() - start_time;

    let mut times = (0..=1).map(|i| {
        println!("- Part {}:", i + 1);
        let cloned = parsed.clone();
        let start_time = Instant::now();
        let result = if i == 0 {
            T::first(cloned)
        } else {
            T::second(cloned)
        };
        let elapsed = Instant::now() - start_time;
        println!("-- Result:\n{}", result);
        if result != results[i] {
            println!(
                "\x1b[1;31mResults do not match! Stored result:\x1b[0m\n{}",
                results[i]
            );
        }
        elapsed
    });

    (parsed_time, times.next().unwrap(), times.next().unwrap())
}

fn run_result(result: &DayResult) -> (Duration, Duration, Duration) {
    let day = result.day;
    let name = result.get_name();
    let file = result.get_file();
    header(&format!(" Day {} ", name));
    match_days!(25)
}

fn header(header: &str) {
    println!("\n\x1b[2;30;47m{:#^60}\x1b[0m", header);
}

fn run_days(days: Vec<u8>) {
    let mut timings: Vec<(String, (Duration, Duration, Duration))> = Vec::new();
    for day in days {
        RESULTS
            .into_iter()
            .filter(|r| r.day == day)
            .for_each(|r| timings.push((r.get_name(), run_result(&r))));
    }
    timings = timings
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&(b.1 .0 + b.1 .1 + b.1 .2), &(a.1 .0 + a.1 .1 + a.1 .2)))
        .collect();

    header(" TIMINGS ");
    for chunk in timings.chunks(10) {
        print!("Day:     ");
        for (name, _) in chunk {
            print!("|{:^15}", name);
        }
        print!("|\nParsing: ");
        for (_name, (a, _, _)) in chunk {
            print!("|{:^15}", format!("{:?}", a));
        }
        print!("|\nPart 1:  ");
        for (_name, (_, b, _)) in chunk {
            print!("|{:^15}", format!("{:?}", b));
        }
        print!("|\nPart 2:  ");
        for (_name, (_, _, c)) in chunk {
            print!("|{:^15}", format!("{:?}", c));
        }
        println!("|");
        println!();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            run_days((1..=25).into_iter().collect());
        }
        2 => match args[1].parse() {
            Ok(day) if (1..=25).contains(&day) => {
                run_days(vec![day]);
            }
            _ => panic!("Wrong day passed"),
        },
        3 => {
            let part = match args[2].parse() {
                Ok(p) if p > 0 => Some(p),
                _ => None,
            };
            match args[1].parse() {
                Ok(day) if (1..=25).contains(&day) => {
                    if let Some(result) =
                        RESULTS.into_iter().find(|r| r.day == day && r.part == part)
                    {
                        run_result(&result);
                    } else {
                        panic!("Wrong part passed");
                    }
                }
                _ => panic!("Wrong day passed"),
            };
        }
        _ => panic!("Wrong parameters"),
    }
}
