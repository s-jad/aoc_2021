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
            let sx: isize = if x.0 > x.1 {
                -1
            } else if x.0 < x.1 {
                1
            } else {
                0
            };
            let sy: isize = if y.0 > y.1 {
                -1
            } else if y.0 < y.1 {
                1
            } else {
                0
            };
            let diff_x = (x.0 as isize - x.1 as isize).abs();
            let diff_y = (y.0 as isize - y.1 as isize).abs();
            let m_d = (diff_x + diff_y).abs();

            if diff_x == 0 {
                for i in 0..=diff_y {
                    coord_vec.push((x.0, (y.0 as isize + (i * sy)) as usize));
                }
            } else if diff_y == 0 {
                for i in 0..=diff_x {
                    coord_vec.push(((x.0 as isize + (i * sx)) as usize, y.0));
                }
            } else {
                for i in 0..=(m_d / 2) {
                    coord_vec.push((
                        (x.0 as isize + (i * sx)) as usize,
                        (y.0 as isize + (i * sy)) as usize,
                    ));
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
