use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn first_part() -> String {
    let file =
        File::open("./src/year2025/exo4/exo4.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            line.expect("Puzzle input must have valid lines")
                .chars()
                .collect()
        })
        .collect();

    let mut response = 0;
    grid.iter().enumerate().for_each(|(y_index, y)| {
        y.iter().enumerate().for_each(|(x_index, x)| {
            if *x == '.' {
                return;
            }
            if fewer_than4_adjacents(&grid, x_index, y_index) {
                println!("Pickable roll: {}/{}", x_index, y_index);
                response += 1;
            }
        });
    });

    format!("Numbers of rolls of paper usable : {}", response)
}

const ADJACENT_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn fewer_than4_adjacents(grid: &Vec<Vec<char>>, x_index: usize, y_index: usize) -> bool {
    println!("Searching at position {}/{}", x_index, y_index);
    let mut adjacent_numbers = 0;
    for (x_offset, y_offset) in ADJACENT_OFFSETS {
        let adjacent_y = y_index as i32 + y_offset;
        let adjacent_y = usize::try_from(adjacent_y);
        if let Ok(adjacent_y) = adjacent_y {
            let adjacent_x = x_index as i32 + x_offset;
            let adjacent_x = usize::try_from(adjacent_x);
            if let Ok(adjacent_x) = adjacent_x {
                if let Some(line) = grid.get(adjacent_y) {
                    if let Some(cell) = line.get(adjacent_x) {
                        if *cell == '@' {
                            adjacent_numbers += 1;
                        }
                    }
                }
            }
        }
    }
    if adjacent_numbers >= 4 {
        return false;
    }
    true
}

pub fn second_part() -> String {
    let file =
        File::open("./src/year2025/exo4/exo4.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            line.expect("Puzzle input must have valid lines")
                .chars()
                .collect()
        })
        .collect();

    let mut response = 0;
    let mut is_done = false;
    while is_done == false {
        let mut new_grid: Vec<Vec<char>> = grid.clone();
        grid.iter().enumerate().for_each(|(y_index, y)| {
            y.iter().enumerate().for_each(|(x_index, x)| {
                if *x == '.' {
                    return;
                }
                if fewer_than4_adjacents(&grid, x_index, y_index) {
                    println!("Pick-able roll: {}/{}", x_index, y_index);
                    let new_cell = new_grid.get_mut(y_index).unwrap().get_mut(x_index).unwrap();
                    *new_cell = '.';
                    response += 1;
                }
            });
        });
        if grid.eq(&new_grid) {
            is_done = true;
        } else {
            grid = new_grid.clone();
        }
    }

    format!("Numbers of rolls of paper usable : {}", response)
}
