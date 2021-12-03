use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Settings {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}

fn part_1(input: &str) -> usize {
    let values = input
        .lines()
        .map(|line| line.parse::<usize>())
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();
    let mut acc = 0;
    let mut prev = values[0];
    for &i in values.iter().skip(1) {
        if i > prev {
            acc += 1;
        }
        prev = i;
    }
    acc
}

fn main() {
    let settings = Settings::from_args();
    dbg!(&settings);
    let input = std::fs::read_to_string(settings.input).expect("Failed to read input file!");
    dbg!(part_1(&input));
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
}
