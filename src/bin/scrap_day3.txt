

fn _place_wire(
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
fn _place_run(
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
                new_current_point = (starting_point.0 + amount, starting_point.1);
            }
        }
        _ => panic!("Bad direction in a run"),
    }
    (new_panel, new_current_point)
}
