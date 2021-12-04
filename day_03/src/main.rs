use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Settings {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}

struct Gamma {
    value: usize,
    bits: usize,
}

impl Gamma {
    fn epsilon(&self) -> usize {
        (!self.value) & ((1 << self.bits) - 1)
    }
}

fn gamma(input: &str) -> Gamma {
    let lines: Vec<_> = input.lines().collect();
    let mut counts = vec![(0, 0); lines[0].len()];
    for line in lines.iter() {
        for (i, c) in line.chars().enumerate() {
            if c == '0' {
                counts[i].0 += 1;
            } else {
                counts[i].1 += 1;
            }
        }
    }
    let mut ret = 0;
    for (i, count) in counts.iter().enumerate() {
        if count.1 > count.0 {
            ret |= 1 << ((counts.len() - 1) - i);
        }
    }
    Gamma {
        value: ret,
        bits: counts.len(),
    }
}

fn part_1(input: &str) -> usize {
    let gamma = gamma(input);
    let epsilon = gamma.epsilon();
    gamma.value * epsilon
}

fn part_2(input: &str) -> isize {
    unimplemented!()
}

fn main() {
    let settings = Settings::from_args();
    dbg!(&settings);
    let input = std::fs::read_to_string(settings.input).expect("Failed to read input file!");
    dbg!(part_1(&input));
    // dbg!(part_2(&input));
}

mod tests {
    #[test]
    fn part_1() {
        let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
        assert_eq!(crate::part_1(input), 198);
    }
    #[test]
    fn part_2() {}
}
