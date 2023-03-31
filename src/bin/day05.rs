use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let first_element = 10099;
    let parsed_first_element = parse_first_element(first_element);
    println!(
        "opcode: {}\nMode 1st: {}\nMode 2nd: {}\nMode 3rd: {}",
        parsed_first_element.opcode,
        parsed_first_element.mode_of_1st_parameter,
        parsed_first_element.mode_of_2nd_parameter,
        parsed_first_element.mode_of_3rd_parameter
    );
}

struct FirstElement {
    opcode: usize,
    mode_of_1st_parameter: usize,
    mode_of_2nd_parameter: usize,
    mode_of_3rd_parameter: usize,
}

fn parse_first_element(first_element: usize) -> FirstElement {
    let mut first_element_as_vec: Vec<char> = from_int_to_string_vector(first_element);
    if first_element_as_vec.len() < 6 {
        first_element_as_vec.insert(0, '0');
    }
    println!("Padded first element is {:?}", first_element_as_vec);

    let opcode: usize = first_element_as_vec[4]
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap();
    let opcode = if opcode == 9 { 99 } else { opcode };
    println!("op code is {:?}", opcode);
    let mode_of_1st_parameter = first_element_as_vec[2]
        .to_string()
        .parse::<usize>()
        .unwrap();
    let mode_of_2nd_parameter = first_element_as_vec[1]
        .to_string()
        .parse::<usize>()
        .unwrap();
    let mode_of_3rd_parameter = first_element_as_vec[0]
        .to_string()
        .parse::<usize>()
        .unwrap();

    //[opcode, mode_of_1st_parameter, mode_of_2nd_parameter, mode_of_3rd_parameter]
    FirstElement {
        opcode,
        mode_of_1st_parameter,
        mode_of_2nd_parameter,
        mode_of_3rd_parameter,
    }
}

fn from_int_to_string_vector(num: usize) -> Vec<char> {
    num.to_string().chars().collect()
}

fn _process_instruction(
    instruction: [usize; 4],
    entire_program: &mut Vec<usize>,
) -> Option<Vec<usize>> {
    entire_program[instruction[3]] = match instruction[0] {
        1 => entire_program[instruction[1]] + entire_program[instruction[2]],
        2 => entire_program[instruction[1]] * entire_program[instruction[2]],
        99 => return None,
        _ => panic!("Found invalid instruction!"),
    };
    Some(entire_program.to_vec())
}

// fn process_entire_program(mut program_vec: Vec<usize>) -> Vec<usize> {
//     for opcode_num in 0..(program_vec.len() / 4) {
//         let first_position_of_this_opcode = opcode_num * 4;
//         let opcode: [usize; 4] = [
//             program_vec[first_position_of_this_opcode],
//             program_vec[first_position_of_this_opcode + 1],
//             program_vec[first_position_of_this_opcode + 2],
//             program_vec[first_position_of_this_opcode + 3],
//         ];
//         program_vec = match process_opcode(opcode, &mut program_vec) {
//             Some(program) => program,
//             None => break,
//         }
//     }
//     program_vec
// }

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
fn can_process_multi_code_programs() {
    let program_string = "1,9,10,3,2,3,11,0,99,30,40,50".to_string();

    let program_vec: Vec<usize> =
        parse_cs_string_of_integers(program_string).expect("Error parsing input from file");

    let processed = process_entire_program(program_vec);
    assert_eq!(
        processed,
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
}

#[test]
fn can_process_multi_code_programs2() {
    let program_string = "1,1,1,4,99,5,6,0,99".to_string();
    let program_vec: Vec<usize> =
        parse_cs_string_of_integers(program_string).expect("Error parsing input from file");
    let processed = process_entire_program(program_vec);
    assert_eq!(processed, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

#[test]
fn can_process_multi_code_programs3() {
    let program_string = "2,4,4,5,99,0".to_string();
    let program_vec: Vec<usize> =
        parse_cs_string_of_integers(program_string).expect("Error parsing input from file");
    let processed = process_entire_program(program_vec);
    assert_eq!(processed, vec![2, 4, 4, 5, 99, 9801]);
}

#[test]
fn can_solve_part_1() {
    assert_eq!(6730673, run_it_given_noun_and_verb(12, 2)[0]);
}

#[test]
fn can_read_vector_of_integers_from_cs_file() {
    let file_name = "inputs/day02.txt";
    let opcode_string: String = read_string_from_file(file_name).unwrap();
    let opcode_vec: Vec<usize> = parse_cs_string_of_integers(opcode_string).unwrap();
    assert_eq!(opcode_vec[3], 3);
    assert_eq!(opcode_vec[opcode_vec.len() - 1], 0);
}
