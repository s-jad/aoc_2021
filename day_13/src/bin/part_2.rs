use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn process(input: &str) -> usize {
    let (points, folds) = input.split_once("\n\n").unwrap();

    let coords = points
        .lines()
        .map(|l| {
            l.split_terminator(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .collect_vec();

    let folds = folds
        .lines()
        .map(|l| (l.as_bytes()[11] as char, l[13..].parse::<usize>().unwrap()))
        .collect_vec();

    let mut coords_set = HashSet::new();

    for i in 0..coords.len() {
        let mut new_x;
        let mut new_y;
        let mut final_coord = coords[i];
        for fold in folds.iter() {
            match fold.0 {
                'x' => {
                    if final_coord.0 > fold.1 {
                        new_x = fold.1 * 2 - final_coord.0
                    } else {
                        new_x = final_coord.0;
                    }
                    final_coord = (new_x, final_coord.1);
                }
                'y' => {
                    if final_coord.1 > fold.1 {
                        new_y = fold.1 * 2 - final_coord.1;
                    } else {
                        new_y = final_coord.1;
                    }
                    final_coord = (final_coord.0, new_y);
                }
                _ => {}
            }
        }
        coords_set.insert(final_coord);
    }

    let (mut max_x, mut max_y) = (0, 0);

    for (x, y) in coords_set.iter() {
        if x > &max_x {
            max_x = *x;
        }
        if y > &max_y {
            max_y = *y;
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            if coords_set.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    1
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
