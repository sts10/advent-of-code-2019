use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

// Some helpful functions when doing Advent of Code challenges
//
// See tests below for examples of usage.

// Reads a multi-line text file into a Vector.
// The Vector will be whatever Rust type that Rust's `parse` function gets from the file.
// (from https://github.com/sts10/eyeoh/blob/master/src/lib.rs#L33-L50)
pub fn read_by_line<T: FromStr>(file_path: &str) -> io::Result<Vec<T>> {
    let mut vec = Vec::new();
    let f = File::open(file_path.trim_matches(|c| c == '\'' || c == ' '))?;
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line?.parse() {
            Ok(l) => vec.push(l),
            Err(_e) => {
                // this is definitely not great, but the Error generated here is of type FromStr, rather than io, so
                // it'd be difficult to return
                panic!("Error reading a line of the file");
            }
        }
    }
    Ok(vec)
}

// If the input file is only one line with tons of characters
// we're going to need to iterate through, this function is handy.
// It reads a text file into a Vector of `char`s (characters),
// which is usually what we want when doing AoC challenges.
pub fn read_string_from_file_to_vector(file_path: &str) -> io::Result<Vec<char>> {
    let mut f = File::open(file_path.trim_matches(|c| c == '\'' || c == ' '))?;
    let mut string_from_file = String::new();
    f.read_to_string(&mut string_from_file)
        .expect("something went wrong reading the file");

    let mut vector_of_chars = Vec::new();
    for c in string_from_file.chars() {
        vector_of_chars.push(c);
    }
    Ok(vector_of_chars)
}

// I found myself often wanting to split a string slice (`&str`)
// by another string slice and get a vector back.
pub fn split_and_vectorize<'a>(string_to_split: &'a str, splitter: &str) -> Vec<&'a str> {
    // let split = string_to_split.split(splitter);
    // split.collect::<Vec<&str>>()
    string_to_split.split(splitter).collect()
}
// To do: have the splitter be a vector of splitters so you found just do `split_and_vectorize(event_string, vec![" ", ":"])[5]` to get `hours`.

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
