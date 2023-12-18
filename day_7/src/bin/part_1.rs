use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut crab_pos = input
        .split_terminator(&[',', '\n'][..])
        .map(|s| s.parse::<usize>().expect("num"))
        .collect_vec();

    crab_pos.sort();
    let crab_len = crab_pos.len();
    let median = if crab_len % 2 == 1 {
        crab_pos[crab_len / 2]
    } else {
        (crab_pos[crab_len / 2 - 1] + crab_pos[crab_len / 2]) / 2
    };

    println!("median => {:?}", median);

    crab_pos
        .iter()
        .map(move |c| (*c as isize - median as isize).abs() as usize)
        .sum()
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
