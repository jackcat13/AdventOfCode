use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn first_part() -> String {
    let file = File::open("./year2025/exo1/exo1.txt").expect("Please provide valid puzzle input");
    let reader = BufReader::new(file);

    let mut dial_pointer = 50;
    let mut password = 0;
    println!("The dial starts by pointing at {}", dial_pointer);
    reader.lines().for_each(|line| {
        let line = line.expect("Please provide valid content in puzzle input");
        let mut chars = line.chars();
        let direction = match chars.next() {
            Some('R') => DialDirection::Right,
            Some('L') => DialDirection::Left,
            None => todo!(),
            _ => todo!(),
        };
        let number_string = chars.collect::<String>();
        let number = i32::from_str_radix(&number_string, 10)
            .expect("Failed to get number from puzzle input");
        match direction {
            DialDirection::Left => dial_pointer = dial_pointer - number,
            DialDirection::Right => dial_pointer = (dial_pointer + number) % 100,
        };
        while dial_pointer < 0 {
            dial_pointer = 100 + dial_pointer;
        }
        println!(
            "The dial is rotated {}{} to point at {}",
            direction, number, dial_pointer
        );
        if dial_pointer == 0 {
            password += 1;
        }
    });

    format!("Password is {}", password)
}

pub fn second_part() -> String {
    let file = File::open("./year2025/exo1/exo1.txt").expect("Please provide valid puzzle input");
    let reader = BufReader::new(file);

    let mut dial_pointer = 50;
    let mut password = 0;
    println!("The dial starts by pointing at {}", dial_pointer);
    reader.lines().for_each(|line| {
        let line = line.expect("Please provide valid content in puzzle input");
        let mut chars = line.chars();
        let direction = match chars.next() {
            Some('R') => DialDirection::Right,
            Some('L') => DialDirection::Left,
            None => todo!(),
            _ => todo!(),
        };
        let number_string = chars.collect::<String>();
        let number_int = i32::from_str_radix(&number_string, 10)
            .expect("Failed to get number from puzzle input");
        let revolutions_number = number_int / 100;
        password += revolutions_number;
        let number = number_int - (revolutions_number * 100);
        let init_dial_pointer = dial_pointer;
        match direction {
            DialDirection::Left => dial_pointer = dial_pointer - number,
            DialDirection::Right => dial_pointer = dial_pointer + number,
        };
        let mut is_zero = false;
        if dial_pointer < 0 {
            dial_pointer = 100 + dial_pointer;
            if init_dial_pointer != 0 {
                password += 1;
                is_zero = true;
            }
        } else if dial_pointer > 99 {
            dial_pointer = dial_pointer - 100;
            password += 1;
            is_zero = true;
        } else if dial_pointer == 0 {
            password += 1;
            is_zero = true;
        }
        println!(
            "The dial is rotated {}{} to point at {} with revolutions number {} and passed by 0 is {}",
            direction, number_int, dial_pointer, revolutions_number, is_zero
        );
    });

    format!("Password is {}", password)
}

enum DialDirection {
    Left,
    Right,
}

impl Display for DialDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialDirection::Left => write!(f, "L"),
            DialDirection::Right => write!(f, "R"),
        }
    }
}
