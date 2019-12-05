use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    println!("Part 1: {}", run_it_given_noun_and_verb(12, 2)[0]);

    for this_noun in 0..99 {
        for this_verb in 0..99 {
            let this_run = run_it_given_noun_and_verb(this_noun, this_verb);
            if this_run[0] == 19_690_720 {
                println!(
                    "Found it!\nNoun is {}; Verb is {}\n100 * noun + verb, and thus the answer to part 2 is {}",
                    this_noun,
                    this_verb,
                    100 * this_noun + this_verb
                );
            }
        }
    }
}

fn run_it_given_noun_and_verb(noun: usize, verb: usize) -> Vec<usize> {
    let file_name = "inputs/day05.txt";
    let program_string: String = read_string_from_file(file_name).expect("Error reading file");
    let mut program_vec: Vec<usize> =
        parse_cs_string_of_integers(program_string).expect("Error parsing input from file");
    program_vec[1] = noun;
    program_vec[2] = verb;

    process_entire_program(program_vec)
}

fn process_entire_program(mut program_vec: Vec<usize>) -> Vec<usize> {
    // First, build an array of size 4 for each opcode (more efficient than vectors)
    for opcode_num in 0..(program_vec.len() / 4) {
        let first_position_of_this_opcode = opcode_num * 4;
        let opcode: [usize; 4] = [
            program_vec[first_position_of_this_opcode],
            program_vec[first_position_of_this_opcode + 1],
            program_vec[first_position_of_this_opcode + 2],
            program_vec[first_position_of_this_opcode + 3],
        ];
        program_vec = match process_opcode(opcode, &mut program_vec) {
            Some(program) => program,
            None => break,
        }
    }
    program_vec
}

fn process_opcode(opcode: [usize; 4], entire_program: &mut Vec<usize>) -> Option<Vec<usize>> {
    entire_program[opcode[3]] = match opcode[0] {
        1 => entire_program[opcode[1]] + entire_program[opcode[2]],
        2 => entire_program[opcode[1]] * entire_program[opcode[2]],
        99 => return None,
        _ => panic!("Found invalid opcode!"),
    };
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
