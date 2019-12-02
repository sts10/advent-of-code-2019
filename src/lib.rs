use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

// from https://github.com/sts10/eyeoh/blob/master/src/lib.rs#L33-L50
pub fn read_by_line<T: FromStr>(file_path: &str) -> io::Result<Vec<T>> {
    let mut vec = Vec::new();
    let f = match File::open(file_path.trim_matches(|c| c == '\'' || c == ' ')) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line?.parse() {
            Ok(l) => vec.push(l),
            Err(_e) => {
                panic!("Error reading a line of the file");
            }
        }
    }
    Ok(vec)
}

pub fn read_string_from_file_to_vector(file_path: &str) -> io::Result<Vec<char>> {
    let mut f = match File::open(file_path.trim_matches(|c| c == '\'' || c == ' ')) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let mut string_from_file = String::new();
    f.read_to_string(&mut string_from_file)
        .expect("something went wrong reading the file");

    let mut vector_of_chars = Vec::new();
    for c in string_from_file.chars() {
        vector_of_chars.push(c);
    }
    Ok(vector_of_chars)
}

// See tests below for examples of usage.
pub fn split_and_vectorize<'a>(string_to_split: &'a str, splitter: &str) -> Vec<&'a str> {
    // let split = string_to_split.split(splitter);
    // split.collect::<Vec<&str>>()
    string_to_split.split(splitter).collect()
}

#[test]
fn can_read_integers_from_file() {
    let file_name = "inputs/day01.txt";
    let masses: Vec<usize> = read_by_line(file_name).unwrap();

    assert_eq!(masses[2], 105685);
    assert_eq!(masses[4], 133339);
}

#[test]
fn can_split_and_vectorize() {
    let event_string = "Guard 13 fell asleep at 7:56";

    let guard_id: usize = split_and_vectorize(event_string, " ")[1].parse().unwrap();
    assert_eq!(guard_id, 13);

    // can nest calls to dial in a bit more
    let hour: u32 = split_and_vectorize(split_and_vectorize(event_string, " ")[5], ":")[0]
        .parse()
        .unwrap();
    assert_eq!(hour, 7);

    // Take multiple elements from a split with one call (but 2 lines, including the join)
    let verb_phrase = &split_and_vectorize(event_string, " ")[2..=3].join(" ");
    assert_eq!(verb_phrase, "fell asleep");
}
