use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    // let wire1 = vec!["R8", "U5", "L5", "D3"];
    // let wire2 = vec!["U7", "R6", "D4", "L4"];
    let wire1 = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
    let wire2 = vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
    let mut panel: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
    let starting_point = (1000 / 2, 1000 / 2);
    println!("Attempting to place wire1");
    panel = place_wire(wire1, 1, panel, starting_point);
    println!("Attempting to place wire2");
    panel = place_wire(wire2, 8, panel, starting_point);

    // println!("Panel is now {:?}", panel);
    // for row in panel.iter() {
    //     println!("{:?}", row);
    // }
    let min_cp = find_min_cross_point_manhattan_distance(panel, 8, starting_point);
    println!("Found min cross point to be {}", min_cp);
}

fn place_wire(
    wire: Vec<&str>,
    number_to_place: usize,
    mut panel: [[usize; 1000]; 1000],
    arbitrary_central_point: (usize, usize),
) -> [[usize; 1000]; 1000] {
    let mut new_starting_point = arbitrary_central_point;
    let mut runner_counter = 0;
    for run in wire {
        let (generated_panel, generated_new_starting_point) =
            place_run(run, number_to_place, panel, new_starting_point);
        panel = generated_panel;
        new_starting_point = generated_new_starting_point;
        runner_counter += 1;
        println!("Run counter is now {}", runner_counter);
    }
    panel[new_starting_point.0][new_starting_point.1] = number_to_place;
    panel
}
fn place_run(
    run: &str,
    number_to_place: usize,
    panel: [[usize; 1000]; 1000],
    starting_point: (usize, usize),
) -> ([[usize; 1000]; 1000], (usize, usize)) {
    let (direction, amount) = get_direction_and_amount(run);
    // let mut new_panel = panel.clone();
    let mut new_panel = panel;
    let mut new_current_point: (usize, usize) = starting_point;
    match direction {
        'R' => {
            for n in 0..amount {
                new_panel[starting_point.0][starting_point.1 + n] += number_to_place;
            }
            new_current_point = (starting_point.0, starting_point.1 + amount);
        }
        'L' => {
            for n in 0..amount {
                // try 3 dots?
                new_panel[starting_point.0][starting_point.1 - n] += number_to_place;
            }
            new_current_point = (starting_point.0, starting_point.1 - amount);
        }
        'U' => {
            for n in 0..amount {
                new_panel[starting_point.0 - n][starting_point.1] += number_to_place;
            }
            new_current_point = (starting_point.0 - amount, starting_point.1);
        }
        'D' => {
            for n in 0..amount {
                new_panel[starting_point.0 + n][starting_point.1] += number_to_place;
            }
            new_current_point = (starting_point.0 + amount, starting_point.1);
        }
        _ => panic!("Bad direction in a run"),
    }
    (new_panel, new_current_point)
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
    panel: [[usize; 1000]; 1000],
    value_to_hunt_for: usize,
    starting_point: (usize, usize),
) -> isize {
    let cross_points = find_all_cross_points(panel, value_to_hunt_for);
    let mut min_cross_point_manhattan_distance = 100000;
    for cross_point in cross_points {
        let this_cross_point_manhattan_distance =
            get_manhattan_distance(starting_point, cross_point);
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
    panel: [[usize; 1000]; 1000],
    value_to_hunt_for: usize,
) -> Vec<(usize, usize)> {
    let mut cross_points: Vec<(usize, usize)> = vec![];
    let mut row_number = 0;
    for row in panel.iter() {
        let mut column_number = 0;
        for _square in row.into_iter() {
            if panel[row_number][column_number] > value_to_hunt_for
                && panel[row_number][column_number] % value_to_hunt_for != 0
            {
                println!(
                    "found a new cross-point at {}, {}",
                    row_number, column_number
                );
                cross_points.push((row_number, column_number));
            }
            column_number += 1;
        }
        row_number += 1;
    }
    cross_points
}
fn get_manhattan_distance(a: (usize, usize), b: (usize, usize)) -> isize {
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
