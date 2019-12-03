use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let file_name = "inputs/day02.txt";
    let program_string: String = read_string_from_file(file_name).expect("Error reading file");
    let program_vec: Vec<usize> =
        parse_cs_string_of_integers(program_string).expect("Error parsing input from file");

    let finished_program = process_entire_program(program_vec);

    println!("Finished program is {:?}", finished_program);

    println!("Value at position 0 is {}", finished_program[0]);
    println!("I've already guessed 509871");
}
fn process_entire_program(mut program_vec: Vec<usize>) -> Vec<usize> {
    // build vec of opcodes from chunks of 4
    for opcode_num in 0..(program_vec.len() / 4) {
        let first_position_of_this_opcode = opcode_num * 4;
        let opcode = vec![
            program_vec[first_position_of_this_opcode],
            program_vec[first_position_of_this_opcode + 1],
            program_vec[first_position_of_this_opcode + 2],
            program_vec[first_position_of_this_opcode + 3],
        ];
        program_vec = match process_opcode(opcode.to_vec(), &mut program_vec) {
            Some(program) => program,
            None => break,
        }
    }
    program_vec
}

// fn process_entire_program(mut program_vec: Vec<usize>) -> Vec<usize> {
//     // build vec of opcodes from chunks of 4
//     let mut vec_of_opcodes: Vec<Vec<usize>> = vec![];
//     for opcode in program_vec.chunks(4) {
//         vec_of_opcodes.push(opcode.to_vec());
//     }
//     for opcode in vec_of_opcodes {
//         program_vec = match process_opcode(opcode, program_vec.clone()) {
//             Some(program) => program,
//             None => break,
//         };
//         vec_of_opcodes = vec![];
//         println!("---- cleared ----");
//         for opcode in program_vec.chunks(4) {
//             vec_of_opcodes.push(opcode.to_vec());
//             println!("Just added {:?}", opcode);
//         }
//     }
//     program_vec
// }
fn process_opcode(opcode: Vec<usize>, entire_program: &mut Vec<usize>) -> Option<Vec<usize>> {
    println!(
        "running opcode {:?} on this program state: {:?}",
        opcode, entire_program
    );
    if opcode[0] == 1 {
        entire_program[opcode[3]] = entire_program[opcode[1]] + entire_program[opcode[2]];
    } else if opcode[0] == 2 {
        entire_program[opcode[3]] = entire_program[opcode[1]] * entire_program[opcode[2]];
    } else if opcode[0] == 99 {
        return None;
    } else {
        panic!("Found invalid opcode!");
    }
    Some(entire_program.to_vec())
}

fn read_string_from_file(file_path: &str) -> io::Result<String> {
    let mut f = File::open(file_path.trim_matches(|c| c == '\'' || c == ' '))?;
    let mut string_from_file = String::new();
    f.read_to_string(&mut string_from_file)
        .expect("something went wrong reading the file");
    Ok(string_from_file)
}

fn parse_cs_string_of_integers(s: String) -> Result<Vec<usize>, std::num::ParseIntError> {
    let vec: Vec<&str> = s.split(',').collect();

    let mut vector_of_integers = Vec::new();
    for num_as_string in vec {
        vector_of_integers.push(num_as_string.trim_end().parse::<usize>()?);
    }
    Ok(vector_of_integers)
}

#[test]
fn can_process_multi_code_programs() {
    let program_string = "1,9,10,3,2,3,11,0,99,30,40,50".to_string();

    let mut program_vec: Vec<usize> =
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
    let mut program_vec: Vec<usize> =
        parse_cs_string_of_integers(program_string).expect("Error parsing input from file");
    let processed = process_entire_program(program_vec);
    assert_eq!(processed, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

#[test]
fn can_process_multi_code_programs3() {
    let program_string = "2,4,4,5,99,0".to_string();
    let mut program_vec: Vec<usize> =
        parse_cs_string_of_integers(program_string).expect("Error parsing input from file");
    let processed = process_entire_program(program_vec);
    assert_eq!(processed, vec![2, 4, 4, 5, 99, 9801]);
}

#[test]
fn can_read_vector_of_integers_from_cs_file() {
    let file_name = "inputs/day02.txt";
    let opcode_string: String = read_string_from_file(file_name).unwrap();
    let opcode_vec: Vec<usize> = parse_cs_string_of_integers(opcode_string).unwrap();
    assert_eq!(opcode_vec[3], 3);
    assert_eq!(opcode_vec[opcode_vec.len() - 1], 0);
}
