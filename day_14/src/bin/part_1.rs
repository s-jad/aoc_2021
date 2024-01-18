use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn process(input: &str) -> usize {
    let mut start = input.lines().nth(0).unwrap().trim().chars().collect_vec();
    let map = input.lines().skip(2).fold(HashMap::new(), |mut acc, l| {
        let parts = l.chars().filter(|c| c.is_alphabetic()).collect_vec();
        acc.insert((parts[0], parts[1]), parts[2]);
        acc
    });

    for step in 0..10 {
        let mut insertions = Vec::new();

        for (idx, win) in start.iter().tuple_windows::<(&char, &char)>().enumerate() {
            if let Some(insert) = map.get(&(*win.0, *win.1)) {
                insertions.push((insert, idx + 1));
            }
        }

        for (offset, (insert, idx)) in insertions.into_iter().enumerate() {
            start.insert(idx + offset, *insert);
        }
    }

    let counts = start.into_iter().counts();

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    max - min
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
