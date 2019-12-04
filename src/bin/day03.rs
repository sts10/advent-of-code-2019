use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    // let wire1 = vec!["R8", "U5", "L5", "D3"];
    // let wire2 = vec!["U7", "R6", "D4", "L4"];
    let wire1 = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
    let wire2 = vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];

    let wire1_as_coordinates = make_coordinates(wire1);
    println!("wire1_as_coordinates is {:?}", wire1_as_coordinates);
    let wire2_as_coordinates = make_coordinates(wire2);
    println!("wire2_as_coordinates is {:?}", wire2_as_coordinates);

    // println!("Panel is now {:?}", panel);
    // for row in panel.iter() {
    //     println!("{:?}", row);
    // }
    let min_cp =
        find_min_cross_point_manhattan_distance(wire1_as_coordinates, wire2_as_coordinates);
    println!("Found min cross point to be {}", min_cp);
}

fn make_coordinates(instructions: Vec<&str>) -> Vec<(isize, isize)> {
    let mut this_wire_coordinates = vec![(0 as isize, 0 as isize)];

    for instruction in instructions {
        let new_starting_point: (isize, isize) =
            this_wire_coordinates[this_wire_coordinates.len() - 1];
        println!("nwe start is {:?}", new_starting_point);
        let (direction, amount) = get_direction_and_amount(instruction);
        match direction {
            'R' => {
                for n in 0..amount {
                    this_wire_coordinates
                        .push((new_starting_point.0, new_starting_point.1 + n as isize));
                }
                // new_starting_point = this_wire_coordinates[this_wire_coordinates.len() - 1];
            }
            'L' => {
                for n in 0..amount {
                    this_wire_coordinates
                        .push((new_starting_point.0, new_starting_point.1 - n as isize));
                }
            }
            'U' => {
                for n in 0..amount {
                    this_wire_coordinates
                        .push((new_starting_point.0 - n as isize, new_starting_point.1));
                }
            }
            'D' => {
                for n in 0..amount {
                    this_wire_coordinates
                        .push((new_starting_point.0 + n as isize, new_starting_point.1));
                }
            }
            _ => panic!("Bad direction in a run"),
        }
    }
    this_wire_coordinates
}
fn get_direction_and_amount(run: &str) -> (char, usize) {
    let run_as_vec: Vec<char> = run.chars().collect();
    // println!("run_as_vec is {:?}", run_as_vec);
    let direction = run_as_vec[0];
    let amount = &run_as_vec[1..run_as_vec.len()];
    let amount: Vec<String> = amount.into_iter().map(|c| c.to_string()).collect();
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
            if wire1_coordinate.0 == wire2_coordinate.0 && wire1_coordinate.1 == wire2_coordinate.1
            {
                println!(
                    "1 is {:?}; 2 is {:?}. Adding {:?}",
                    wire1_coordinate, wire2_coordinate, wire1_coordinate
                );
                cross_points.push(wire1_coordinate);
            }
        }
    }
    cross_points
}
fn get_manhattan_distance(a: (isize, isize), b: (isize, isize)) -> isize {
    // isize::abs((a.0 - b.0) as isize) + isize::abs((a.1 - b.1) as isize)
    isize::abs((a.0 as isize - b.0 as isize) as isize)
        + isize::abs((a.1 as isize - b.1 as isize) as isize)
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
