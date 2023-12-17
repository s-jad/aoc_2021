use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> isize {
    let mut start = (0, 0);

    let steps = input
        .lines()
        .map(|l| {
            let (dir, num) = l
                .split_whitespace()
                .collect_tuple()
                .expect("Should be tuple");
            match dir {
                "forward" => (num.parse::<isize>().expect("should be number"), 0),
                "up" => (0, -num.parse::<isize>().expect("should be number")),
                "down" => (0, num.parse::<isize>().expect("should be number")),
                _ => (0, 0),
            }
        })
        .collect_vec();

    let mut aim = 0;
    for step in steps.into_iter() {
        aim += step.1;

        if step.1 == 0 {
            start.0 += step.0;
            start.1 += step.0 * aim;
        }
    }
    start.0 * start.1
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
