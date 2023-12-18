use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_terminator(" | ")
                .collect_tuple::<(_, _)>()
                .expect("is tuple")
                .1
        })
        .flat_map(|s| s.split_terminator(" ").collect_vec())
        .filter(|s| s.len() != 5 && s.len() != 6)
        .count()
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
