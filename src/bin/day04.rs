use advent_of_code_2019::read_by_line;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut number_of_different_passwords_that_meet_the_criteria_for_part_1 = 0;
    let mut number_of_different_passwords_that_meet_the_criteria_for_part_2 = 0;

    for password in 109165..576723 {
        if meets_criteria_for_part_1(&from_int_to_string_vector(password)) {
            number_of_different_passwords_that_meet_the_criteria_for_part_1 += 1;
        }
        if meets_criteria_for_part_2(&from_int_to_string_vector(password)) {
            number_of_different_passwords_that_meet_the_criteria_for_part_2 += 1;
        }
    }
    println!(
        "Part 1: Found {} possible passwords that meet the criteria",
        number_of_different_passwords_that_meet_the_criteria_for_part_1
    );
    println!(
        "Part 2: Found {} possible passwords that meet the criteria",
        number_of_different_passwords_that_meet_the_criteria_for_part_2
    );
    // println!(
    //     "password as vector is {:?}",
    //     from_int_to_string_vector(first_password_to_try)
    // );
    // contains_a_double(&from_int_to_string_vector(first_password_to_try));
}

fn meets_criteria_for_part_1(password: &[char]) -> bool {
    contains_a_double(&password) && is_six_digits(&password) && never_decreases(&password)
}

fn meets_criteria_for_part_2(password: &[char]) -> bool {
    contains_at_least_one_clean_double(&password)
        && contains_a_double(&password)
        && is_six_digits(&password)
        && never_decreases(&password)
}
fn contains_a_double(password: &[char]) -> bool {
    for d in password.windows(2) {
        // println!("d is {:?}", d);
        if d[0] == d[1] {
            return true;
        }
    }
    false
}

fn is_six_digits(password: &[char]) -> bool {
    password.len() == 6
}

fn never_decreases(password: &[char]) -> bool {
    for d in password.windows(2) {
        if d[0] > d[1] {
            return false;
        }
    }
    true
}

fn contains_at_least_one_clean_double(password: &[char]) -> bool {
    for s in password.windows(4) {
        if s[0] != s[1] && s[1] == s[2] && s[2] != s[3] {
            return true;
        }
    }
    // End cases
    if password[3] != password[4] && password[4] == password[5] {
        return true;
    }
    if password[0] == password[1] && password[1] != password[2] {
        return true;
    }
    false
}

fn count_doubles(password: &[char]) -> usize {
    let mut number_of_doubles = 0;
    for d in password.windows(2) {
        if d[0] == d[1] {
            number_of_doubles += 1;
        }
    }
    number_of_doubles
}

fn count_triples(password: &[char]) -> usize {
    let mut number_of_triples = 0;
    for d in password.windows(3) {
        if d[0] == d[1] && d[1] == d[2] {
            number_of_triples += 1;
        }
    }
    number_of_triples
}
fn from_int_to_string_vector(num: usize) -> Vec<char> {
    num.to_string().chars().collect()
}

fn from_string_vector_to_int(vec: Vec<char>) -> usize {
    let vec: Vec<String> = vec.iter().map(|c| c.to_string()).collect();
    vec.join("").parse::<usize>().unwrap()
}

#[test]
fn can_do_the_part_two_crtieria_test() {
    assert!(contains_at_least_one_clean_double(
        &from_int_to_string_vector(112233)
    ));
    assert_eq!(
        contains_at_least_one_clean_double(&from_int_to_string_vector(123444)),
        false
    );
    assert!(contains_at_least_one_clean_double(
        &from_int_to_string_vector(111122)
    ));
}
