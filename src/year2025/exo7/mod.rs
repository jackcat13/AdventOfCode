use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn first_part() -> String {
    let file =
        File::open("./src/year2025/exo7/exo7.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            let line = line.expect("Puzzle must have valid lines");
            line.chars().collect()
        })
        .collect();

    let grid_mut = &mut grid;
    let grid_mut_clone = grid_mut.clone();

    let mut split_numbers = 0;
    for line_index in 0..grid_mut_clone.clone().len() {
        for cell_index in 0..grid_mut_clone.clone().get(line_index).unwrap().len() {
            let grid_mut_clone = grid_mut.clone();
            let cell = grid_mut_clone
                .get(line_index)
                .unwrap()
                .get(cell_index)
                .unwrap();
            match cell {
                'S' => spawn_beam_at_position(line_index + 1, cell_index, grid_mut),
                '|' => {
                    process_beam_next_position(line_index, cell_index, grid_mut, &mut split_numbers)
                }
                '^' => continue,
                '.' => continue,
                _ => panic!("Invalid grid value"),
            }
        }
    }

    for line in grid {
        println!("{:?}", line);
    }

    format!("Numbers of splits : {}", split_numbers)
}

fn spawn_beam_at_position(line_index: usize, cell_index: usize, grid: &mut Vec<Vec<char>>) {
    if let Some(next_line) = grid.get_mut(line_index) {
        if let Some(next_position) = next_line.get_mut(cell_index) {
            *next_position = '|';
        }
    }
}

fn process_beam_next_position(
    line_index: usize,
    cell_index: usize,
    grid: &mut Vec<Vec<char>>,
    split_numbers: &mut i32,
) {
    let line_index = line_index + 1;
    if let Some(next_line) = grid.get_mut(line_index) {
        if let Some(next_position) = next_line.get_mut(cell_index) {
            match next_position {
                'S' => panic!("S should be the beam generator, can't be reached by beam"),
                '.' => *next_position = '|',
                '^' => split_beam_from_position(line_index, cell_index, grid, split_numbers),
                '|' => (),
                _ => panic!("Invalid cell value"),
            };
        }
    }
}

fn split_beam_from_position(
    line_index: usize,
    cell_index: usize,
    grid: &mut Vec<Vec<char>>,
    split_numbers: &mut i32,
) {
    *split_numbers += 1;
    spawn_beam_at_position(line_index, cell_index - 1, grid);
    spawn_beam_at_position(line_index, cell_index + 1, grid);
}

#[derive(Clone, Debug)]
struct BeamPointer {
    line_index: usize,
    cell_index: usize,
    value: u64,
}

pub fn second_part() -> String {
    let file =
        File::open("./src/year2025/exo7/exo7.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| {
            let line = line.expect("Puzzle must have valid lines");
            line.chars().collect()
        })
        .collect();

    let mut beam_pointers: Vec<BeamPointer> = vec![];
    let mut beam_line_index = 0;

    for line_index in 0..grid.len() {
        for cell_index in 0..grid.get(line_index).unwrap().len() {
            let cell = grid.get(line_index).unwrap().get(cell_index).unwrap();
            match cell {
                'S' => {
                    beam_line_index = line_index + 1;
                    beam_pointers.push(BeamPointer {
                        line_index: beam_line_index,
                        cell_index: cell_index,
                        value: 1,
                    });
                    break;
                }
                _ => (),
            }
        }
    }

    for line_index in beam_line_index..grid.len() {
        beam_pointers = beam_pointers
            .clone()
            .iter()
            .filter(|bp| bp.line_index > line_index - 1)
            .cloned()
            .collect();
        print!("Pointers end of line : ");
        for pointer in beam_pointers.clone() {
            print!("{:?} ; ", pointer)
        }
        println!("");
        let mut new_pointers: Vec<BeamPointer> = vec![];
        for beam_pointer_index in 0..beam_pointers.clone().len() {
            let beam_pointer_ref = beam_pointers.get_mut(beam_pointer_index).unwrap();
            let line_index_next = beam_pointer_ref.line_index + 1;
            if let Some(next_line) = grid.get(line_index_next) {
                let next_cell = next_line.get(beam_pointer_ref.cell_index).unwrap();
                match next_cell {
                    '^' => {
                        let left_cell_index = beam_pointer_ref.cell_index - 1;
                        split_beam_to_position(
                            &mut new_pointers,
                            line_index_next,
                            left_cell_index,
                            beam_pointer_ref.value,
                        );
                        let right_cell_index = beam_pointer_ref.cell_index + 1;
                        split_beam_to_position(
                            &mut new_pointers,
                            line_index_next,
                            right_cell_index,
                            beam_pointer_ref.value,
                        );
                    }
                    '.' => {
                        beam_pointer_ref.line_index += 1;
                    }
                    _ => (),
                }
            }
        }
        for new_pointer in new_pointers {
            beam_pointers.push(new_pointer.clone());
        }
    }

    format!(
        "Numbers of timelines: {}",
        beam_pointers.clone().iter().map(|bp| bp.value).sum::<u64>()
    )
}

fn split_beam_to_position(
    beam_pointers: &mut Vec<BeamPointer>,
    line_index: usize,
    left_cell_index: usize,
    existing_value: u64,
) {
    if let Some(other_pointer) = beam_pointers.iter_mut().find(|other_pointer| {
        other_pointer.line_index == line_index && other_pointer.cell_index == left_cell_index
    }) {
        other_pointer.value += existing_value;
    } else {
        beam_pointers.push(BeamPointer {
            line_index: line_index,
            cell_index: left_cell_index,
            value: existing_value,
        });
    }
}
