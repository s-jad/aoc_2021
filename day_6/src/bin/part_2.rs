use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut day_arr = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    let fishies = input
        .split_terminator(&[',', '\n'][..])
        .map(|s| s.parse::<usize>().expect("number"))
        .collect_vec();

    for fish in fishies {
        day_arr[fish] += 1;
    }

    let total_days = 256;

    for _ in 0..total_days {
        let fish_ready_to_reproduce = day_arr[0];

        for j in 0..8 {
            day_arr[j] = day_arr[j + 1];
        }

        day_arr[6] += fish_ready_to_reproduce;
        day_arr[8] = fish_ready_to_reproduce;
    }

    day_arr.iter().sum()
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
