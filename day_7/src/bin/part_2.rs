use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let crab_pos = input
        .split_terminator(&[',', '\n'][..])
        .map(|s| s.parse::<isize>().expect("num"))
        .collect_vec();

    let crab_len = crab_pos.len();
    let mean = (crab_pos.iter().sum::<isize>() as f64 / crab_len as f64).ceil() as isize;

    let mut final_total: f64 = 999999999f64;

    for i in -1..=1 {
        let total = crab_pos
            .iter()
            .map(move |c| {
                ((*c - (mean + i)).abs() as f64 / 2f64) * (1f64 + (*c - (mean + i)).abs() as f64)
            })
            .sum::<f64>();

        if total <= final_total {
            final_total = total;
        }
    }

    final_total as usize
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
