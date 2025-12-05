use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn first_part() -> String {
    let file =
        File::open("./src/year2025/exo5/exo5.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let mut fresh_ranges: Vec<(u64, u64)> = vec![];
    let mut ids: Vec<u64> = vec![];
    let mut has_done_parsing_ranges = false;

    for line in reader.lines() {
        let line = line.expect("Puzzle inputs must have valid line");

        if line.is_empty() {
            has_done_parsing_ranges = true;
            continue;
        }

        if has_done_parsing_ranges == false {
            let mut range = line.split("-");
            let (first, second) = (
                range.next().expect("Must have a first value in range"),
                range.next().expect("Must have a second value in range"),
            );
            let first = u64::from_str_radix(first, 10).expect("Range id fist must be integer");
            let second = u64::from_str_radix(second, 10).expect("Range id second must be integer");
            fresh_ranges.push((first, second));
        } else {
            let id = u64::from_str_radix(line.as_str(), 10).expect("Id must be integer");
            ids.push(id);
        }
    }

    let response = ids
        .iter()
        .filter(|id| {
            fresh_ranges
                .iter()
                .any(|range| **id >= range.0 && **id <= range.1)
        })
        .count();

    format!("Number of fresh products : {}", response)
}

pub fn second_part() -> String {
    let file =
        File::open("./src/year2025/exo5/exo5.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let mut fresh_ranges: Vec<(u64, u64)> = vec![];

    for line in reader.lines() {
        let line = line.expect("Puzzle inputs must have valid line");

        if line.is_empty() {
            break;
        }

        let mut range = line.split("-");
        let (first, second) = (
            range.next().expect("Must have a first value in range"),
            range.next().expect("Must have a second value in range"),
        );
        let first = u64::from_str_radix(first, 10).expect("Range id fist must be integer");
        let second = u64::from_str_radix(second, 10).expect("Range id second must be integer");
        fresh_ranges.push((first, second));
    }

    fresh_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ranges_iter = fresh_ranges.iter_mut();
    let mut previous_range = ranges_iter.next().unwrap();
    while let Some(range) = ranges_iter.next() {
        if range.0 <= previous_range.1 {
            range.0 = previous_range.1 + 1;
        }
        previous_range = range;
    }

    let response = fresh_ranges
        .iter()
        .map(|range| {
            let count = (range.0..=range.1).count();
            println!("Range {:?} has size : {}", range, count);
            count
        })
        .sum::<usize>();

    format!("Number of fresh products : {}", response)
}
