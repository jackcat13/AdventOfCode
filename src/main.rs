use std::env;

mod year2025;

fn main() -> Result<(), u8> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return usage();
    }
    let year = args.get(1).unwrap().to_owned();
    let day_and_part = args.get(2).unwrap().to_owned();

    let result = match year.as_str() {
        "2025" => match day_and_part.as_str() {
            "1-1" => year2025::exo1::first_part(),
            "1-2" => year2025::exo1::second_part(),
            "2-1" => year2025::exo2::first_part(),
            "2-2" => year2025::exo2::second_part(),
            "3-1" => year2025::exo3::first_part(),
            "3-2" => year2025::exo3::second_part(),
            _ => return not_found(year, day_and_part),
        },
        _ => return not_found(year, day_and_part),
    };

    println!("Result of the puzzle : {}", result);

    Ok(())
}

fn usage() -> Result<(), u8> {
    println!("USAGE : cargo run <year> <day-and-part; eg: 1-1>");
    Err(1)
}

fn not_found(year: String, day_and_part: String) -> Result<(), u8> {
    println!(
        "Could not find year / day-and-part : {} / {}. Please look at existing ones under year folders.",
        year, day_and_part
    );
    Err(2)
}
