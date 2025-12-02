use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn first_part() -> String {
    let file =
        File::open("./src/year2025/exo2/exo2.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let line = lines
        .next()
        .expect("Puzzle input must have a line")
        .expect("Puzzle input must have a line");

    let mut invalid_ids: Vec<u64> = vec![];
    let mut solution: u64 = 0;
    line.split(",").for_each(|range_id| {
        let mut ids = range_id.split("-");
        let id1 = ids
            .next()
            .expect("Range id 1st value must be present in puzzle input");

        let id2 = ids
            .next()
            .expect("Range id 2nd value must be present in puzzle input");

        let id1 = u64::from_str_radix(id1, 10).expect("Range id 1st must be a valid id");
        let id2 = u64::from_str_radix(id2, 10).expect("Range id 2nd must be a valid id");

        println!("Starting searching between range : {}-{}", id1, id2);

        for i in id1..=id2 {
            if is_valid_id(i) == false {
                invalid_ids.push(i);
                solution += i;
            }
        }
    });

    println!("Invalid ids are {:?}", invalid_ids);
    format!("Puzzle solution is {}", solution)
}

fn is_valid_id(id: u64) -> bool {
    let id_string = id.to_string();
    let id_length = id_string.len();
    if id_length % 2 != 0 {
        return true;
    }

    let middle = id_length / 2;
    let id_first_half = &id_string[0..middle];
    let id_second_half = &id_string[middle..id_length];

    if id_first_half.eq(id_second_half) {
        return false;
    }

    return true;
}

pub fn second_part() -> String {
    let file =
        File::open("./src/year2025/exo2/exo2.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let line = lines
        .next()
        .expect("Puzzle input must have a line")
        .expect("Puzzle input must have a line");

    let mut invalid_ids: Vec<u64> = vec![];
    let mut solution: u64 = 0;
    line.split(",").for_each(|range_id| {
        let mut ids = range_id.split("-");
        let id1 = ids
            .next()
            .expect("Range id 1st value must be present in puzzle input");

        let id2 = ids
            .next()
            .expect("Range id 2nd value must be present in puzzle input");

        let id1 = u64::from_str_radix(id1, 10).expect("Range id 1st must be a valid id");
        let id2 = u64::from_str_radix(id2, 10).expect("Range id 2nd must be a valid id");

        println!("Starting searching between range : {}-{}", id1, id2);

        for i in id1..=id2 {
            if is_valid_id_second(i) == false {
                invalid_ids.push(i);
                solution += i;
            }
        }
    });

    println!("Invalid ids are {:?}", invalid_ids);
    format!("Puzzle solution is {}", solution)
}

fn is_valid_id_second(id: u64) -> bool {
    let id_string = id.to_string();
    let id_length = id_string.len();
    let middle = id_length / 2;

    for i in 0..middle {
        let pattern = &id_string[0..i + 1];
        if id_length % pattern.len() != 0 {
            continue;
        }
        let mut j = 0;
        let mut is_matching = true;
        while j < id_length {
            if id_string[j..j + pattern.len()].eq(pattern) == false {
                is_matching = false;
            }
            j += pattern.len();
        }
        if is_matching == false {
            continue;
        }
        return false;
    }

    return true;
}
