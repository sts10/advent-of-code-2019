use advent_of_code_2019::read_by_line;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    // let wire1 = vec!["R8", "U5", "L5", "D3"]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect();
    // let wire2 = vec!["U7", "R6", "D4", "L4"]
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect();
    // let wire1 = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
    // let wire2 = vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
    let wire1 = vec![
        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let wire2 = vec![
        "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    // let file_name = "inputs/day03.txt";
    // let wires_as_vec: Vec<String> = read_by_line(file_name).unwrap();
    // let wire1 = split_cs_string(wires_as_vec[0].clone());
    // let wire2 = split_cs_string(wires_as_vec[1].clone());

    let wire1_as_coordinates = make_coordinates(wire1);
    let wire2_as_coordinates = make_coordinates(wire2);

    // let min_cp =
    //     find_min_cross_point_manhattan_distance(wire1_as_coordinates, wire2_as_coordinates);
    // println!("Found min cross point to be {}", min_cp);
    // println!("841 and 839 are too high!");

    let part_2_solution = solve_part_2(wire1_as_coordinates, wire2_as_coordinates);
    println!("part 2 solution is: {}", part_2_solution);
    println!("12338 is too high!");
}

fn make_coordinates(instructions: Vec<String>) -> Vec<(isize, isize)> {
    let mut this_wire_coordinates = vec![(0 as isize, 0 as isize)];

    for instruction in instructions {
        let new_starting_point: (isize, isize) =
            this_wire_coordinates[this_wire_coordinates.len() - 1];
        let (direction, amount) = get_direction_and_amount(&instruction);
        match direction {
            'R' => {
                for n in 1..=amount {
                    this_wire_coordinates
                        .push((new_starting_point.0, new_starting_point.1 + n as isize));
                }
            }
            'L' => {
                for n in 1..=amount {
                    this_wire_coordinates
                        .push((new_starting_point.0, new_starting_point.1 - n as isize));
                }
            }
            'U' => {
                for n in 1..=amount {
                    this_wire_coordinates
                        .push((new_starting_point.0 - n as isize, new_starting_point.1));
                }
            }
            'D' => {
                for n in 1..=amount {
                    this_wire_coordinates
                        .push((new_starting_point.0 + n as isize, new_starting_point.1));
                }
            }
            _ => panic!("Bad direction in a run"),
        }
    }
    println!("this wire's coordinates: {:?}", this_wire_coordinates);
    this_wire_coordinates
}

fn get_direction_and_amount(run: &str) -> (char, usize) {
    let run_as_vec: Vec<char> = run.chars().collect();
    // println!("run_as_vec is {:?}", run_as_vec);
    let direction = run_as_vec[0];
    let amount = &run_as_vec[1..run_as_vec.len()];
    let amount: Vec<String> = amount.iter().map(|c| c.to_string()).collect();
    let amount = amount.to_vec().join("").parse::<usize>().unwrap();
    (direction, amount)
}

fn find_min_cross_point_manhattan_distance(
    wire1: Vec<(isize, isize)>,
    wire2: Vec<(isize, isize)>,
) -> isize {
    let cross_points = find_all_cross_points(wire1, wire2);
    println!("cross points are {:?}", cross_points);
    let mut min_cross_point_manhattan_distance = 100000;
    for cross_point in cross_points {
        let this_cross_point_manhattan_distance =
            get_manhattan_distance((0 as isize, 0 as isize), cross_point);
        if this_cross_point_manhattan_distance == 0 {
            continue;
        }
        println!(
            "Found a cross-point manhattan distance of {}",
            this_cross_point_manhattan_distance
        );
        if this_cross_point_manhattan_distance < min_cross_point_manhattan_distance {
            min_cross_point_manhattan_distance = this_cross_point_manhattan_distance;
        }
    }
    min_cross_point_manhattan_distance
}

fn find_all_cross_points(
    wire1: Vec<(isize, isize)>,
    wire2: Vec<(isize, isize)>,
) -> Vec<(isize, isize)> {
    let mut cross_points: Vec<(isize, isize)> = vec![];
    for wire1_coordinate in wire1 {
        for wire2_coordinate in &wire2 {
            // if wire1_coordinate.0 == wire2_coordinate.0 && wire1_coordinate.1 == wire2_coordinate.1
            if wire1_coordinate == *wire2_coordinate {
                if !(wire1_coordinate.0 == 0 && wire1_coordinate.1 == 0) {
                    cross_points.push(wire1_coordinate);
                }
            }
        }
    }
    cross_points
}

fn solve_part_2(wire1: Vec<(isize, isize)>, wire2: Vec<(isize, isize)>) -> usize {
    let mut cross_points: Vec<(isize, isize, usize)> = vec![];
    let mut wire1_step_counter = 0;
    for wire1_coordinate in wire1 {
        let mut wire2_step_counter = 0;
        for wire2_coordinate in &wire2 {
            if wire1_coordinate == *wire2_coordinate {
                if !(wire1_coordinate.0 == 0 && wire1_coordinate.1 == 0) {
                    // "including the intersection being considered"
                    wire1_step_counter += 1;
                    wire2_step_counter += 1;
                    cross_points.push((
                        wire1_coordinate.0,
                        wire1_coordinate.1,
                        wire1_step_counter + wire2_step_counter,
                    ));
                }
            }
            wire2_step_counter += 1;
        }
        wire1_step_counter += 1;
    }
    // now find cross point with fewest number of steps
    let mut min_cross_point_steps = 100000;
    for cross_point in cross_points {
        if cross_point.2 < min_cross_point_steps {
            min_cross_point_steps = cross_point.2;
        }
    }
    min_cross_point_steps - 2 // for the (0,0) at the beginning
}

fn get_manhattan_distance(a: (isize, isize), b: (isize, isize)) -> isize {
    isize::abs((a.0 as isize - b.0 as isize) as isize)
        + isize::abs((a.1 as isize - b.1 as isize) as isize)
}

fn split_cs_string(s: String) -> Vec<String> {
    let mut vector_of_strings = Vec::new();
    for instruction in s.split(',') {
        vector_of_strings.push(instruction.trim_end().to_string());
    }
    vector_of_strings
}

fn _read_string_from_file(file_path: &str) -> io::Result<String> {
    let mut f = File::open(file_path.trim_matches(|c| c == '\'' || c == ' '))?;
    let mut string_from_file = String::new();
    f.read_to_string(&mut string_from_file)
        .expect("something went wrong reading the file");
    Ok(string_from_file)
}
