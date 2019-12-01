use std::fs::File;
use std::io::{BufRead, BufReader};

// Part 1 code.

fn add_fuel_one(mass: u32) -> u32 {
    let fuel = mass / 3 - 2;
    return fuel;
}

// Part 2 code.

fn add_fuel_two(mass: u32) -> u32 {
    let mut fuel = 0;

    match (mass / 3).checked_sub(2) {
        None => fuel = 0,
        Some(sub) => fuel = sub
    }

    if fuel == 0 {
        return fuel;
    }
    else {
        return fuel + add_fuel_two(fuel);
    }
}

fn main() {
    const FILE_NAME: &str = "day1_input.txt";

    println!("Receiving file {}", FILE_NAME);

    let file = File::open(FILE_NAME).expect("Could not find or open file in root directory.");
    let reader = BufReader::new(file);
    let mut fuel_total:u32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let mass: u32 = line.unwrap().parse().unwrap();
        let required_fuel: u32 = add_fuel_two(mass);
        fuel_total += required_fuel;
    }

    println!("Total fuel required: {}", fuel_total);
}
