#![feature(str_split_once)]

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Settings {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}

fn part_1(input: &str) -> isize {
    let mut horizontal = 0;
    let mut depth = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (command, value) = line
            .split_once(' ')
            .unwrap_or_else(|| panic!("failed to split a line!"));
        let value = value
            .parse::<isize>()
            .expect("part two of a command was not a number!");
        match command {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("invalid command!"),
        }
    }
    horizontal * depth
}

fn part_2(input: &str) -> isize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (command, value) = line
            .split_once(' ')
            .unwrap_or_else(|| panic!("failed to split a line!"));
        let value = value
            .parse::<isize>()
            .expect("part two of a command was not a number!");
        match command {
            "forward" => {
                horizontal += value;
                depth += aim * value
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("invalid command!"),
        }
    }
    horizontal * depth
}

fn main() {
    let settings = Settings::from_args();
    dbg!(&settings);
    let input = std::fs::read_to_string(settings.input).expect("Failed to read input file!");
    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

mod tests {
    #[test]
    fn part_1() {
        let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
        assert_eq!(crate::part_1(input), 150);
    }
    #[test]
    fn part_2() {
        let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
        assert_eq!(crate::part_2(input), 900);
    }
}
