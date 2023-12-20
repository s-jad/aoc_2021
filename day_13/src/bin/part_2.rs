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
    let mut final_max_y = max_y;
    let mut final_max_x = max_x;

    for i in 0..coords.len() {
        let mut new_x;
        let mut new_y;
        let mut mx_x = max_x;
        let mut mx_y = max_y;
        let mut final_coord = coords[i];
        println!("coords : {:?}", coords[i]);
        println!("mx_x = {:?}", mx_x);
        println!("mx_y = {:?}", mx_y);
        for fold in folds.iter() {
            println!("fold {:?} at {:?}", fold.0, fold.1);
            match fold.0 {
                "x" => {
                    mx_x = fold.1 - 1;
                    if final_coord.0 > fold.1 {
                        new_x = fold.1 * 2 - final_coord.0
                    } else {
                        new_x = final_coord.0;
                    }
                    final_coord = (new_x, final_coord.1);
                }
                "y" => {
                    mx_y = fold.1 - 1;

                    if final_coord.1 > fold.1 {
                        new_y = fold.1 * 2 - final_coord.1;
                    } else {
                        new_y = final_coord.1;
                    }
                    final_coord = (final_coord.0, new_y);
                }
                _ => {}
            }
            println!("new coord => {:?}\n", final_coord);
        }
        final_max_x = mx_x;
        final_max_y = mx_y;
        coords_set.insert(final_coord);
    }

    let mut grid = vec![vec!['.'; final_max_x + 1]; final_max_y + 1];

    for y in 0..=final_max_y {
        for x in 0..=final_max_x {
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
