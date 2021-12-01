use text_io::read;

fn main() {
    solve_day_1();
}

fn solve_day_1() {
    let measurements = get_raw_measurements();
    let result1 = solve_day_1_part_1(&measurements);
    println!("Result of part 1: {result1}");
    let result2 = solve_day_1_part_2(&measurements);
    println!("Result of part 2: {result2}");
}
fn solve_day_1_part_1(puzzle_input: &Vec<i32>) -> u32 {
    get_increases_count(puzzle_input)
}
fn solve_day_1_part_2(puzzle_input: &Vec<i32>) -> u32 {
    let moving_sum: Vec<i32> = puzzle_input
        .windows(3)
        .map(|wnd| wnd.iter().sum())
        .collect();
    get_increases_count(&moving_sum)
}

fn get_increases_count<T: Ord>(measurements: &Vec<T>) -> u32 {
    let mut previous = None;
    let mut increases: u32 = 0;
    for current_measurement in measurements {
        if let Some(previous_measurement) = previous {
            if current_measurement > previous_measurement {
                increases += 1;
            }
        }
        previous = Some(current_measurement);
    }
    increases
}

fn get_raw_measurements() -> Vec<i32> {
    println!("Give me your input:");
    let measurements = {
        let mut measurements: Vec<i32> = vec![];
        let mut input: String = read!();
        while let Ok(n) = input.parse::<i32>() {
            measurements.push(n);
            input = read!();
        }
        measurements
    };
    measurements
}
