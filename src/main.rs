use clap::{App, Arg};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod helpers;

fn main() -> Result<(), String> {
    let matches = App::new("Advent of Code")
        .author("Laura")
        .about("Advent of code solutions")
        .arg(
            Arg::with_name("day")
                .short("d")
                .takes_value(true)
                .help(r#"Day number."#),
        )
        .get_matches();

    Ok(match matches.value_of("day") {
        Some("1") => {
            let data_01 = helpers::read_file_to_int("src/day01/input.txt");
            println!(
                "Task 1: {}, Task 2: {}",
                day01::task1(&data_01),
                day01::task2(&data_01)
            );
        }
        Some("2") => {
            let data_02 = helpers::read_file_to_string("src/day02/input.txt");
            println!(
                "Task 1: {}, Task 2: {}",
                day02::task1(&data_02),
                day02::task2(&data_02)
            );
        }
        Some("3") => {
            let data_03 = helpers::read_file_to_string("src/day03/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day03::task1(&data_03),
                day03::task2(&data_03)
            );
        }
        Some("4") => {
            let data_04 = helpers::read_file_to_string("src/day04/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day04::task1(&data_04),
                day04::task2(&data_04)
            );
        }
        Some("5") => {
            let data_05 = helpers::read_file_to_string("src/day05/input.txt");
            println!(
                "Task 1: {}, Task2: {}",
                day05::d05_task1(&data_05),
                day05::d05_task2(&data_05)
            );
        }
        _ => panic!("No solution exists for this day"),
    })
}
