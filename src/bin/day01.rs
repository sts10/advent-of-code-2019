use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let file_name = "inputs/day01.txt"; // &str (rather than a String)
    let masses: Vec<usize> = read_by_line(file_name).unwrap();
    // let mut total_fuel = 0;
    // for mass in masses {
    //     total_fuel = total_fuel + get_fuel_from_mass(mass);
    // }
    // println!("Part 1 total fuel: {}", total_fuel);
    let mut total_fuel = 0;
    for mass in masses {
        total_fuel = total_fuel + get_fuel_from_mass_part_2(mass);
    }
    println!("Part 2 total fuel: {}", total_fuel);
}

fn get_fuel_from_mass(mass: usize) -> isize {
    // take its mass, divide by three, round down, and subtract 2.

    // if ((mass as f64 / 3.0).floor() as isize) - 2 < 2 {
    //     return 0;
    // }
    ((mass as f64 / 3.0).floor() as isize) - 2
}

fn get_fuel_from_mass_part_2(mass: usize) -> usize {
    let mut total_fuel: usize = 0;
    let mut new_mass: isize = mass as isize;
    loop {
        let this_run: isize = get_fuel_from_mass(new_mass as usize);
        if this_run < 0 {
            break;
        }
        total_fuel = total_fuel + this_run as usize;
        println!("this_run is {}", this_run);
        println!("total_fuel is now {}", total_fuel);
        new_mass = this_run;

        // if this_run <= 2 {
        //     break;
        // }
    }
    total_fuel
}

// from https://github.com/sts10/eyeoh/blob/master/src/lib.rs#L33
fn read_by_line<T: FromStr>(file_path: &str) -> io::Result<Vec<T>> {
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

#[test]
fn can_find_fuel_given_mass() {
    assert_eq!(get_fuel_from_mass(12), 2);
    // assert_eq!(get_fuel_from_mass(5), 2);
    assert_eq!(get_fuel_from_mass(100756), 33583);
}

#[test]
fn can_find_true_fuel_given_mass() {
    assert_eq!(get_fuel_from_mass_part_2(14), 2);
    assert_eq!(get_fuel_from_mass_part_2(1969), 966);
    assert_eq!(get_fuel_from_mass_part_2(100756), 50346);
}
