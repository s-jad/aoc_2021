use itertools::Itertools;
use std::{time::Instant, usize::MAX};

fn process(input: &str) -> usize {
    let crab_pos = input
        .split_terminator(&[',', '\n'][..])
        .map(|s| s.parse::<usize>().expect("num"))
        .collect_vec();

    let crab_len = crab_pos.len();
    let avg = (crab_pos.iter().sum::<usize>() as f64 / crab_len as f64).ceil() as usize;

    let mut best_cost = MAX;
    for i in (avg - (crab_len / 6) as usize)..(avg + (crab_len / 6) as usize) {
        let total_cost: usize = crab_pos
            .iter()
            .map(move |c| (*c as isize - i as isize).abs() as usize)
            .sum();

        if total_cost < best_cost {
            best_cost = total_cost;
        }
    }

    best_cost
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
