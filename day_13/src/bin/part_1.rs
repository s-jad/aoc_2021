use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn process(input: &str) -> usize {
    let data = input
        .split_terminator("\n\n")
        .collect_tuple::<(_, _)>()
        .unwrap();

    let mut coords = data
        .0
        .lines()
        .map(|l| {
            l.split_terminator(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .collect_vec();

    let folds = data
        .1
        .lines()
        .map(|l| {
            let parts = l.split_terminator(&[' ', '='][..]).collect_vec();
            (parts[2], parts[3].parse::<usize>().unwrap())
        })
        .collect_vec();

    let (mut max_x, mut max_y) = (0, 0);

    for (x, y) in coords.iter() {
        if x > &max_x {
            max_x = *x;
        }
        if y > &max_y {
            max_y = *y;
        }
    }

    let mut coords_set = HashSet::new();

    for i in 0..coords.len() {
        let new_x;
        let new_y;

        match folds[0].0 {
            "x" => {
                if coords[i].0 > folds[0].1 {
                    new_x = max_x - coords[i].0;
                } else {
                    new_x = coords[i].0;
                }
                coords_set.insert((new_x, coords[i].1));
            }
            "y" => {
                if coords[i].1 > folds[0].1 {
                    new_y = max_y - coords[i].1;
                } else {
                    new_y = coords[i].1;
                }
                coords_set.insert((coords[i].0, new_y));
            }
            _ => {}
        }
    }

    coords_set.len()
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
