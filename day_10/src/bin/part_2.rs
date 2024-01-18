use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let chunks = input
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();

    let mut totals = Vec::new();

    for chunk in chunks.iter() {
        let mut stack = Vec::new();
        let mut corrupted = false;
        for c in chunk.iter() {
            match c {
                '<' | '[' | '(' | '{' => stack.push(c),
                _ => {
                    let opening = stack.pop().unwrap();
                    if ((*opening as u8) as i32 - (*c as u8) as i32).abs() > 3 {
                        corrupted = true;
                    }
                }
            }
        }

        if !corrupted && stack.len() != 0 {
            let mut total = 0;
            for c in stack.into_iter().rev() {
                match c {
                    '(' => total = (total * 5) + 1,
                    '[' => total = (total * 5) + 2,
                    '{' => total = (total * 5) + 3,
                    '<' => total = (total * 5) + 4,
                    _ => unreachable!(),
                }
            }
            totals.push(total);
        }
    }

    totals.sort();
    let mid = totals.len() / 2;

    totals[mid]
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
