use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Token {
    Number(u64),
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub fn first_part() -> String {
    let file =
        File::open("./src/year2025/exo6/exo6.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let tokens_grid: Vec<Vec<Token>> = reader
        .lines()
        .map(|line| {
            let mut tokens_line = vec![];
            let line = line.expect("Puzzle input must have valid line");
            let mut chars = line.chars();

            while let Some(next_char_val) = chars.next() {
                println!("Parsing char {}", next_char_val);
                if next_char_val.is_numeric() {
                    let mut number_builder = String::new();
                    number_builder.push(next_char_val);
                    for nested_char_val in &mut chars.clone().peekable() {
                        println!("Parsing next values of numeric {}", nested_char_val);
                        if nested_char_val == ' ' {
                            break;
                        } else {
                            number_builder.push(nested_char_val);
                            chars.next();
                        }
                    }
                    tokens_line.push(Token::Number(
                        u64::from_str_radix(number_builder.as_str(), 10)
                            .expect("Invalid input puzzle value, expects numeric"),
                    ));
                } else if next_char_val == ' ' {
                    continue;
                } else {
                    println!("Parsing next operation = {}", next_char_val);
                    let operation = match next_char_val {
                        '+' => Token::Plus,
                        '-' => Token::Minus,
                        '*' => Token::Multiply,
                        '/' => Token::Divide,
                        _ => panic!("Invalid character in puzzle input"),
                    };
                    tokens_line.push(operation);
                }
            }

            tokens_line
        })
        .collect();

    let lines_number = tokens_grid.len();
    let columns_number = tokens_grid
        .get(0)
        .expect("Tokens grid must have a first line")
        .len();
    let mut total = 0;

    println!(
        "Column number : {} and lines number : {}",
        columns_number, lines_number
    );

    for column_index in 0..columns_number {
        let mut column_total = 0;
        let mut numbers_column: Vec<u64> = vec![];
        if let Token::Number(number) = tokens_grid
            .get(0)
            .expect("Token grid must have this line")
            .get(column_index)
            .expect("Token grid must have this column")
        {
            column_total = *number;
        } else {
            panic!("First column value must be a number");
        }
        for line_index in 1..lines_number {
            let cell = tokens_grid
                .get(line_index)
                .expect("Token grid must have this line")
                .get(column_index)
                .expect("Token grid must have this line/column");
            println!("Operating {:?}", cell);
            match cell {
                Token::Number(number) => numbers_column.push(*number),
                Token::Plus => {
                    for number in numbers_column.clone() {
                        column_total += number;
                    }
                }
                Token::Minus => {
                    for number in numbers_column.clone() {
                        column_total -= number;
                    }
                }
                Token::Multiply => {
                    for number in numbers_column.clone() {
                        column_total *= number;
                    }
                }
                Token::Divide => {
                    for number in numbers_column.clone() {
                        column_total /= number;
                    }
                }
            };
        }
        println!("Column total : {}", column_total);
        total += column_total;
    }

    format!("Response of this puzzle is : {total}",)
}

pub fn second_part() -> String {
    let file =
        File::open("./src/year2025/exo6/exo6.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| {
            line.expect("puzzle input must have valid lines")
                .to_string()
        })
        .collect();

    let mut total = 0;

    let lines_number = lines.len();
    let columns_number = lines.get(0).expect("Must have a first line").len();

    let mut numbers: Vec<u64> = vec![];
    for column_index in (0..columns_number).rev() {
        let mut number_builder = String::new();
        for line_index in 0..lines_number {
            let characters = lines
                .get(line_index)
                .expect("Must have valid line")
                .chars()
                .collect::<Vec<char>>();
            let character = characters
                .get(column_index)
                .expect("Must have valid line/column");
            if character.is_numeric() {
                number_builder.push(*character);
            } else if *character == '+'
                || *character == '-'
                || *character == '*'
                || *character == '/'
            {
                total += apply_operation_on_numbers(&mut numbers, &number_builder, *character);
                numbers = vec![];
                number_builder = String::new();
            }
        }

        register_number(&mut numbers, number_builder.as_str());
    }

    format!("Response of the puzzle is {}", total)
}

fn apply_operation_on_numbers(
    numbers: &mut Vec<u64>,
    number_builder: &String,
    operation: char,
) -> u64 {
    register_number(numbers, number_builder.as_str());
    println!("Applying {} operation on current numbers", operation);
    let numbers_clone = numbers.clone();
    let mut numbers_iter = numbers_clone.iter();
    let mut column_total = *numbers_iter
        .next()
        .expect("Operation must have valid values");
    for number in numbers_iter {
        match operation {
            '+' => column_total += number,
            '-' => column_total -= number,
            '*' => column_total *= number,
            '/' => column_total /= number,
            _ => panic!("Invalid operation"),
        };
    }
    println!("Obtaining total column : {}", column_total);
    column_total
}

fn register_number(numbers: &mut Vec<u64>, number_builder: &str) {
    if number_builder.is_empty() == false {
        println!("Registering number {}", number_builder);
        numbers.push(u64::from_str_radix(number_builder, 10).expect("Must be a valid number"));
        println!("Current numbers : {:?}", numbers);
    }
}
