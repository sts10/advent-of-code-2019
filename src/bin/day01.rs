use advent_of_code_2019::read_by_line;
fn main() {
    let file_name = "inputs/day01.txt";
    let masses: Vec<usize> = read_by_line(file_name).unwrap();
    let mut total_fuel_part_1 = 0;
    let mut total_fuel_part_2 = 0;
    for mass in masses {
        total_fuel_part_1 += get_fuel_from_mass(mass);
        total_fuel_part_2 += get_fuel_from_mass_part_2(mass);
    }
    println!("Part 1 total fuel: {}", total_fuel_part_1);
    println!("Part 2 total fuel: {}", total_fuel_part_2);
}

fn get_fuel_from_mass(mass: usize) -> isize {
    // take its mass, divide by three, round down, and subtract 2.
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
        total_fuel += this_run as usize;
        new_mass = this_run;
    }
    total_fuel
}

#[test]
fn can_find_fuel_given_mass() {
    assert_eq!(get_fuel_from_mass(12), 2);
    assert_eq!(get_fuel_from_mass(100756), 33583);
}

#[test]
fn can_find_true_fuel_given_mass() {
    assert_eq!(get_fuel_from_mass_part_2(14), 2);
    assert_eq!(get_fuel_from_mass_part_2(1969), 966);
    assert_eq!(get_fuel_from_mass_part_2(100756), 50346);
}
