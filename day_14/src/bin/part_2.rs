use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn process(input: &str) -> usize {
    let data = input.lines().nth(0).unwrap().trim();

    let last = (data.chars().last().unwrap(), 1);

    let mut start =
        data.chars()
            .tuple_windows::<(_, _)>()
            .fold(HashMap::new(), |mut acc, (a, b)| {
                acc.entry((a, b)).and_modify(|v| *v += 1).or_insert(1);
                acc
            });

    let map = input.lines().skip(2).fold(HashMap::new(), |mut acc, l| {
        let parts = l.chars().filter(|c| c.is_alphabetic()).collect_vec();
        acc.insert(
            (parts[0], parts[1]),
            ((parts[0], parts[2]), (parts[2], parts[1])),
        );
        acc
    });

    for _ in 0..40 {
        let mut new_start = HashMap::new();

        for (pair, count) in start.iter() {
            if let Some((a, b)) = map.get(pair) {
                new_start
                    .entry(*a)
                    .and_modify(|c: &mut usize| *c += *count)
                    .or_insert(*count);
                new_start
                    .entry(*b)
                    .and_modify(|c: &mut usize| *c += *count)
                    .or_insert(*count);
            } else {
                new_start.insert(*pair, *count);
            }
        }

        start = new_start;
    }

    let mut counts =
        start
            .iter()
            .map(|((a1, _), c)| (a1, c))
            .fold(HashMap::new(), |mut acc, (a, c)| {
                acc.entry(a)
                    .and_modify(|v: &mut usize| *v += *c)
                    .or_insert(*c);
                acc
            });

    counts
        .entry(&last.0)
        .and_modify(|v: &mut usize| *v += last.1);

    counts.values().max().unwrap() - counts.values().min().unwrap()
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
