use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn first_part() -> String {
    let file =
        File::open("./src/year2025/exo3/exo3.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let largest_joltages: Vec<u64> = reader
        .lines()
        .map(|line| {
            let line = line.expect("Invalid line from puzzle input");

            let joltages: Vec<u64> = line
                .chars()
                .map(|c| c.to_string())
                .map(|c| {
                    u64::from_str_radix(&c, 10).expect("Invalid value from line of puzzle input")
                })
                .collect();

            println!("Searching max value in line {:?}", joltages);
            let mut current_max = 0;
            joltages.iter().enumerate().for_each(|(index, joltage)| {
                joltages[index + 1..].iter().for_each(|joltage_2| {
                    let combined = format!("{}{}", joltage, joltage_2);
                    let combined_int =
                        u64::from_str_radix(&combined, 10).expect("Must be a valid combined value");
                    if combined_int > current_max {
                        current_max = combined_int;
                    }
                });
            });

            current_max
        })
        .collect();

    println!("Largest joltages selected : {:?}", largest_joltages);
    let response = largest_joltages.iter().fold(0, |acc, &x| acc + x);
    format!("{}", response)
}

const BATTERIES_NUMBER: usize = 12;

pub fn second_part() -> String {
    let file =
        File::open("./src/year2025/exo3/exo3.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let largest_joltages: Vec<u64> = reader
        .lines()
        .map(|line| {
            let line = line.expect("Invalid line from puzzle input");

            let joltages: Vec<u64> = line
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("Values must be integers in puzzle inputs")
                        as u64
                })
                .collect();

            println!("Searching max value in line {:?}", joltages);
            let mut highest = "".to_string();
            let mut begin_search_window = 0;
            for remaining_batteries in (1..=BATTERIES_NUMBER).rev() {
                println!("Remaining batteries : {}", remaining_batteries);
                let mut max_value = 0;
                let end_search_window = joltages.len() - remaining_batteries;
                println!(
                    "Search window is {}/{}",
                    begin_search_window, end_search_window
                );
                let mut max_index = 0;
                joltages[begin_search_window..=end_search_window]
                    .iter()
                    .enumerate()
                    .for_each(|(index_joltage, joltage)| {
                        if max_value < *joltage {
                            max_value = *joltage;
                            max_index = index_joltage;
                        }
                    });
                highest.push_str(max_value.to_string().as_str());
                begin_search_window += max_index + 1;
                println!(
                    "Keep highest value {} at position {}",
                    max_value,
                    begin_search_window - 1
                );
            }
            println!("Selected highest : {}", highest);
            u64::from_str_radix(highest.as_str(), 10)
                .expect("Must be a valid number processed based on puzzle inputs")
        })
        .collect();

    println!("Largest joltages selected : {:?}", largest_joltages);
    let response = largest_joltages.iter().fold(0, |acc, &x| acc + x);
    format!("{}", response)
}
