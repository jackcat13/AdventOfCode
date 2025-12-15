use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Circuits {
    circuits: Vec<Circuit>,
}

impl Circuits {
    fn add_items_to_circuits(&mut self, item1: &Item, item2: &Item) {
        println!(
            "Trying to add item1 {} and item2 {} in circuits.",
            item1.id, item2.id
        );
        for circuit in self.circuits.iter_mut() {
            if circuit.has_the_two_item(item1, item2) {
                println!(
                    "Circuit {} already has the two items, adding nothing",
                    circuit.id
                );
                return;
            }
        }

        let mut to_merge = vec![];
        if let Some(circuit2) = self
            .circuits
            .iter_mut()
            .find(|circuit| circuit.has_item(item2))
        {
            to_merge = circuit2.items.clone();
            circuit2.items = vec![];
        }
        if let Some(circuit1) = self
            .circuits
            .iter_mut()
            .find(|circuit| circuit.has_item(item1))
        {
            for item in to_merge {
                circuit1.items.push(item);
            }
        }
    }

    fn sort_by_size(&mut self) {
        self.circuits
            .sort_by(|a, b| a.items.len().cmp(&b.items.len()));
    }

    fn has_only_one_remaining(&self) -> bool {
        self.circuits.len() == 1
    }

    fn clean_empty(&mut self) {
        let mut empty_indexes = vec![];
        self.circuits.iter().enumerate().for_each(|(i, circuit)| {
            if circuit.items.len() == 0 {
                empty_indexes.push(i);
            }
        });
        for index in empty_indexes {
            self.circuits.remove(index);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Circuit {
    id: usize,
    items: Vec<Item>,
}

impl Circuit {
    fn has_the_two_item(&self, item1: &Item, item2: &Item) -> bool {
        self.items.contains(item1) && self.items.contains(item2)
    }

    fn has_item(&self, item: &Item) -> bool {
        self.items.contains(item)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Distances {
    distances: HashMap<(usize, usize), Distance>,
}

impl Distances {
    fn add_distance_from_items(&mut self, item1: &Item, item2: &Item) {
        let distance = Distance {
            items_tuple: (item1.clone(), item2.clone()),
            distance: Distances::euclidian_distance(item1, item2),
        };
        self.distances.insert((item1.id, item2.id), distance);
    }

    fn any_with_both_items(&self, item1: &Item, item2: &Item) -> bool {
        self.distances.get(&(item1.id, item2.id)).is_some()
            || self.distances.get(&(item2.id, item1.id)).is_some()
    }

    fn euclidian_distance(item1: &Item, item2: &Item) -> i64 {
        i64::isqrt(
            i64::pow(item1.item.x - item2.item.x, 2)
                + i64::pow(item1.item.y - item2.item.y, 2)
                + i64::pow(item1.item.z - item2.item.z, 2),
        )
    }

    fn sort_by_distance(&self) -> Vec<Distance> {
        let mut list: Vec<Distance> = self.distances.iter().map(|(_, d)| d).cloned().collect();
        list.sort_by(|a, b| a.distance.cmp(&b.distance));
        list
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Distance {
    items_tuple: (Item, Item),
    distance: i64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Item {
    id: usize,
    item: Vec3,
}

const CIRCUITS_NUMBERS_TO_PROCESS: usize = 1000;

pub fn first_part() -> String {
    let file =
        File::open("./src/year2025/exo8/exo8.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let items: Vec<Item> = reader
        .lines()
        .enumerate()
        .map(|(index, line)| to_item(index, line.expect("Puzzle input must have valid line")))
        .collect();

    println!("Collected {} items", items.len());
    /*println!("Collected items :");
    for item in items.clone() {
        println!("{:?}", item);
    }*/

    let mut distances: Distances = Distances {
        distances: HashMap::new(),
    };

    for i in 0..items.len() {
        for j in 0..items.len() {
            if i == j {
                continue;
            }
            let item1 = items.get(i).unwrap();
            let item2 = items.get(j).unwrap();

            if distances.any_with_both_items(item1, item2) {
                continue;
            }

            distances.add_distance_from_items(item1, item2);
        }
    }

    println!("Sorting collected distances.");
    let distances = distances.sort_by_distance();
    println!("Collected {} distances", distances.len());

    /*println!("Collected distances :");
    for distance in distances.distances.clone() {
        println!(
            "Items {} ; {} ; with distance {}",
            distance.items_tuple.0.id, distance.items_tuple.1.id, distance.distance
        );
    }*/

    let mut circuits = Circuits {
        circuits: items
            .iter()
            .clone()
            .enumerate()
            .map(|(index, i)| Circuit {
                id: index,
                items: vec![i.clone()],
            })
            .collect::<Vec<Circuit>>(),
    };
    for distance in distances.clone().iter().take(CIRCUITS_NUMBERS_TO_PROCESS) {
        circuits.add_items_to_circuits(&distance.items_tuple.0, &distance.items_tuple.1);
    }
    circuits.sort_by_size();

    let mut circuits_iter = circuits.circuits.iter().rev().clone();
    let next_len = circuits_iter
        .next()
        .expect("Circuits must have a first value")
        .items
        .len();
    println!("Next Circuit has size : {}", next_len);
    let mut response = next_len;
    let next_len = circuits_iter
        .next()
        .expect("Circuits must have a first value")
        .items
        .len();
    println!("Next Circuit has size : {}", next_len);
    response *= next_len;
    let next_len = circuits_iter
        .next()
        .expect("Circuits must have a first value")
        .items
        .len();
    println!("Next Circuit has size : {}", next_len);
    response *= next_len;

    format!(
        "Three largest circuits size multiplied together : {}",
        response
    )
}

fn to_item(index: usize, line: String) -> Item {
    let mut item_split = line.split(",");
    Item {
        id: index,
        item: Vec3 {
            x: i64::from_str_radix(item_split.next().expect("line must have x value"), 10)
                .expect("x value must be number"),
            y: i64::from_str_radix(item_split.next().expect("line must have y value"), 10)
                .expect("y value must be number"),
            z: i64::from_str_radix(item_split.next().expect("line must have z value"), 10)
                .expect("z value must be number"),
        },
    }
}

pub fn second_part() -> String {
    let file =
        File::open("./src/year2025/exo8/exo8.txt").expect("Please provide valid puzzle input");

    let reader = BufReader::new(file);

    let items: Vec<Item> = reader
        .lines()
        .enumerate()
        .map(|(index, line)| to_item(index, line.expect("Puzzle input must have valid line")))
        .collect();

    println!("Collected {} items", items.len());
    /*println!("Collected items :");
    for item in items.clone() {
        println!("{:?}", item);
    }*/

    let mut distances: Distances = Distances {
        distances: HashMap::new(),
    };

    for i in 0..items.len() {
        for j in 0..items.len() {
            if i == j {
                continue;
            }
            let item1 = items.get(i).unwrap();
            let item2 = items.get(j).unwrap();

            if distances.any_with_both_items(item1, item2) {
                continue;
            }

            distances.add_distance_from_items(item1, item2);
        }
    }

    println!("Sorting collected distances.");
    let distances = distances.sort_by_distance();
    println!("Collected {} distances", distances.len());

    /*println!("Collected distances :");
    for distance in distances.distances.clone() {
        println!(
            "Items {} ; {} ; with distance {}",
            distance.items_tuple.0.id, distance.items_tuple.1.id, distance.distance
        );
    }*/

    let mut circuits = Circuits {
        circuits: items
            .iter()
            .clone()
            .enumerate()
            .map(|(index, i)| Circuit {
                id: index,
                items: vec![i.clone()],
            })
            .collect::<Vec<Circuit>>(),
    };

    let distances_clone = distances.clone();
    let mut distances_iter = distances_clone.iter();
    let mut last_connection = (
        Item {
            id: 0,
            item: Vec3 { x: 0, y: 0, z: 0 },
        },
        Item {
            id: 0,
            item: Vec3 { x: 0, y: 0, z: 0 },
        },
    );
    while let Some(distance) = distances_iter.next() {
        circuits.add_items_to_circuits(&distance.items_tuple.0, &distance.items_tuple.1);
        circuits.clean_empty();
        last_connection = (
            distance.items_tuple.0.clone(),
            distance.items_tuple.1.clone(),
        );
        if circuits.has_only_one_remaining() {
            break;
        }
    }
    format!(
        "The connection that resulted in single circuit: {:?}, and multiplication of x coordinates : {}",
        last_connection,
        last_connection.0.item.x * last_connection.1.item.x
    )
}
