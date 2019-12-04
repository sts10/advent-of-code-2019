use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let wire1 = vec!["R8", "U5", "L5", "D3"];
    let wire2 = vec!["U7", "R6", "D4", "L4"];
    let mut panel: [[usize; 30]; 30] = [[0; 30]; 30];
    panel = place_wire(wire1, panel);
    // println!("Panel is now {:?}", panel);
    for row in panel.iter() {
        println!("{:?}", row);
    }
}

fn place_wire(wire: Vec<&str>, mut panel: [[usize; 30]; 30]) -> [[usize; 30]; 30] {
    let arbitrary_central_point = (12, 8);
    let mut new_starting_point = arbitrary_central_point;
    for run in wire {
        // this is giving the error: "invalid left-hand expression"
        (panel, new_starting_point) = place_run(run, &panel, arbitrary_central_point);
    }
    panel
}
fn place_run(
    run: &str,
    panel: &[[usize; 30]; 30],
    starting_point: (usize, usize),
) -> ([[usize; 30]; 30], (usize, usize)) {
    let (direction, amount) = get_direction_and_amount(run);
    let mut new_panel = panel.clone();
    let mut new_current_point: (usize, usize);
    match direction {
        'R' => {
            for n in 1..amount {
                new_panel[starting_point.0][starting_point.1 + n] = 1;
            }
            new_current_point = (starting_point.0, starting_point.1 + amount);
        }
        'L' => {
            for n in 1..amount {
                new_panel[starting_point.0][starting_point.1 - n] = 1;
            }
            new_current_point = (starting_point.0, starting_point.1 - amount);
        }
        'U' => {
            for n in 1..amount {
                new_panel[starting_point.0 - n][starting_point.1] = 1;
            }
            new_current_point = (starting_point.0 - amount, starting_point.1);
        }
        'D' => {
            for n in 1..amount {
                new_panel[starting_point.0 + n][starting_point.1] = 1;
                new_current_point = (starting_point.0 + amount, starting_point.1);
            }
        }
        _ => panic!("Bad direction in a run"),
    }
    (new_panel, new_current_point)
}
fn get_direction_and_amount(run: &str) -> (char, usize) {
    let run_as_vec: Vec<char> = run.chars().collect();
    // println!("run_as_vec is {:?}", run_as_vec);
    let direction = run_as_vec[0];
    let mut amount = &run_as_vec[1..run_as_vec.len()];
    let amount: Vec<String> = amount.into_iter().map(|c| c.to_string()).collect();
    let amount = amount.to_vec().join("").parse::<usize>().unwrap();
    (direction, amount)
}

fn read_string_from_file(file_path: &str) -> io::Result<String> {
    let mut f = File::open(file_path.trim_matches(|c| c == '\'' || c == ' '))?;
    let mut string_from_file = String::new();
    f.read_to_string(&mut string_from_file)
        .expect("something went wrong reading the file");
    Ok(string_from_file)
}

fn parse_cs_string_of_integers(s: String) -> Result<Vec<usize>, std::num::ParseIntError> {
    let mut vector_of_integers = Vec::new();
    for num_as_string in s.split(',') {
        vector_of_integers.push(num_as_string.trim_end().parse::<usize>()?);
    }
    Ok(vector_of_integers)
}

#[test]
fn can_read_vector_of_integers_from_cs_file() {
    let file_name = "inputs/day02.txt";
    let opcode_string: String = read_string_from_file(file_name).unwrap();
    let opcode_vec: Vec<usize> = parse_cs_string_of_integers(opcode_string).unwrap();
    assert_eq!(opcode_vec[3], 3);
    assert_eq!(opcode_vec[opcode_vec.len() - 1], 0);
}
