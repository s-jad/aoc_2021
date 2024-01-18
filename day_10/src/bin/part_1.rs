use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let chunks = input
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();

    let mut stack = Vec::new();

    let mut corrupted = 0;

    for chunk in chunks.iter() {
        for c in chunk.iter() {
            match c {
                '<' | '[' | '(' | '{' => stack.push(c),
                ')' => {
                    let opening = stack.pop().unwrap();
                    if ((*opening as u8) as i32 - (*c as u8) as i32).abs() > 3 {
                        corrupted += 3;
                    }
                }
                ']' => {
                    let opening = stack.pop().unwrap();
                    if ((*opening as u8) as i32 - (*c as u8) as i32).abs() > 3 {
                        corrupted += 57;
                    }
                }
                '}' => {
                    let opening = stack.pop().unwrap();
                    if ((*opening as u8) as i32 - (*c as u8) as i32).abs() > 3 {
                        corrupted += 1197;
                    }
                }
                '>' => {
                    let opening = stack.pop().unwrap();
                    if ((*opening as u8) as i32 - (*c as u8) as i32).abs() > 3 {
                        corrupted += 25137;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    corrupted
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
