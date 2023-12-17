use itertools::Itertools;
use std::collections::HashMap;
use std::time::Instant;

fn count_duplicates(lines: &Vec<(usize, usize)>) -> usize {
    let mut counts = HashMap::new();
    for &coord in lines {
        *counts.entry(coord).or_insert(0) += 1;
    }
    counts.values().filter(|&&count| count > 1).count()
}

fn process(input: &str) -> usize {
    let coords = input
        .lines()
        .map(|l| {
            l.split_terminator(&[' ', '-', '>', ','])
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().expect("should be num"))
                .collect_tuple::<(_, _, _, _)>()
                .expect("should be divisible by 4")
        })
        .map(|t| ((t.0, t.2), (t.1, t.3)))
        .collect_vec();

    let lines = coords
        .into_iter()
        .map(|(x, y)| {
            let mut coord_vec = Vec::new();
            if y.0 == y.1 {
                let max_x = x.0.max(x.1);
                let min_x = x.0.min(x.1);
                for i in min_x..=max_x {
                    coord_vec.push((i, y.0));
                }
            } else if x.0 == x.1 {
                let may_y = y.0.max(y.1);
                let min_y = y.0.min(y.1);
                for i in min_y..=may_y {
                    coord_vec.push((x.0, i));
                }
            }
            coord_vec
        })
        .filter(|v| !v.is_empty())
        .flatten()
        .collect_vec();

    count_duplicates(&lines)
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
