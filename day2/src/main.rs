use std::vec::Vec;
use std::fs;

fn main() {

    const FILE_NAME: &str = "day2_input.csv";

    println!("Attempting to read file {} from root directory", FILE_NAME);

    let reader = fs::read_to_string(FILE_NAME).unwrap();
    let mut int_vec: Vec<usize> = reader
        .trim()
        .split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let mut counter: usize = 0; // Goes up to 4.
    let mut total_count: usize = 0;
    let mut op_array: [usize; 4] = [0; 4];
    let int_vec_copy: Vec<usize> = int_vec.clone();

    for record in int_vec_copy {
        op_array[counter] = record;

        counter += 1;
        if counter == 4 {
            counter = 0;

            if op_array[0] == 1 { // Addition
                let op_result = int_vec[op_array[1]] + int_vec[op_array[2]];
                int_vec[op_array[3]] = op_result;
            }
            else if op_array[0] == 2 { // Multiplication
                let op_result = int_vec[op_array[1]] * int_vec[op_array[2]];
                int_vec[op_array[3]] = op_result;
            }
            else {
                println!("Invalid opcode at position {}: {}", total_count, op_array[0]);
                break;
            }
        }

        total_count += 1;
    }

    println!("New integer array:");
    for int_element in int_vec {
        print!("{} ", int_element);
    }

}
