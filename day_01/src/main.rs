use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Settings {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}

fn part_1(input: &str) -> usize {
    let mut values = input
        .lines()
        .map(|line| line.parse::<usize>())
        .filter_map(|line| line.ok());
    let mut acc = 0;
    let mut prev = values.next().unwrap();
    for i in values {
        if i > prev {
            acc += 1;
        }
        prev = i;
    }
    acc
}

fn part_2(input: &str) -> usize {
    let mut acc = 0;
    let values = input
        .lines()
        .map(|line| line.parse::<usize>())
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();
    for i in 0..values.len() {
        let v4 = values.get(i+3);
        if v4.is_none() {
            break;
        }
        let v1 = values[i];
        let v2 = values[i+1];
        let v3 = values[i+2];
        let v4 = v4.unwrap();
        let a = v1 + v2 + v3;
        let b = v2 + v3 + v4;
        if a < b {
            acc += 1;
        }
    }
    acc
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
        let input = r#"199
200
208
210
200
207
240
269
260
263"#;
        assert_eq!(crate::part_1(input), 7);
    }
    #[test]
    fn part_2() {
        let input = r#"199
200
208
210
200
207
240
269
260
263"#;
        assert_eq!(crate::part_2(input), 5);
    }
}
