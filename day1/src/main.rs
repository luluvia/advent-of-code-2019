use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const FILE_NAME: &str = "day1_input.txt";

    println!("Receiving file {}", FILE_NAME);

    let file = File::open(FILE_NAME).expect("Could not find or open file in root directory.");
    let reader = BufReader::new(file);
    let mut fuel_total:u32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let mass: u32 = line.unwrap().parse().unwrap();
        let required_fuel: u32 = mass / 3 - 2;
        fuel_total += required_fuel;
    }

    println!("Total fuel required: {}", fuel_total);
}
