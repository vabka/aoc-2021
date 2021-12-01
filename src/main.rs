use itertools::Itertools;
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
    let result3 = solve_day_1_part_2_windowless(&measurements);
    println!("Alternative result of part 2: {result3}");
}
// Window 2. Subwindow 1
// Window 4. Subwindo
fn solve_day_1_part_1(puzzle_input: &Vec<i32>) -> usize {
    get_growth_count(puzzle_input.into_iter())
}
fn solve_day_1_part_2(puzzle_input: &Vec<i32>) -> usize {
    let iter = puzzle_input.windows(3).map(|wnd| wnd.iter().sum::<i32>());
    get_growth_count(iter)
}

fn solve_day_1_part_2_windowless(puzzle_input: &Vec<i32>) -> u32 {
    let mut increases: u32 = 0;
    let mut prev: Option<i32> = None;

    let mut i = 0;
    while i < puzzle_input.len() - 2 {
        if let Some(n) = prev {
            if puzzle_input[i + 2] > n {
                increases += 1;
            }
        }
        prev = Some(puzzle_input[i]);
        i += 1;
    }

    increases
}

fn get_growth_count<T, Collection>(iterator: Collection) -> usize
where
    T: Clone + PartialOrd,
    Collection: Iterator<Item = T>,
{
    iterator.tuple_windows().filter(|(p, n)| p < n).count()
}

fn get_raw_measurements() -> Vec<i32> {
    println!("Give me your input (send ^D when complete):");
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
