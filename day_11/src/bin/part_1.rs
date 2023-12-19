use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn induce_neighbours(
    (x, y): (isize, isize),
    coords: &mut HashSet<(isize, isize)>,
    octopii: &mut Vec<Vec<u8>>,
    checked: &mut Vec<(isize, isize)>,
) -> isize {
    if !coords.remove(&(x, y)) || checked.contains(&(x, y)) {
        return 0;
    }

    if octopii[y as usize][x as usize] >= 9 {
        checked.push((x, y));
    }

    1 + [
        (x - 1, y),
        (x + 1, y),
        (x, y + 1),
        (x, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
        (x - 1, y - 1),
    ]
    .iter_mut()
    .map(|&mut (x, y)| {
        if let Some(row) = octopii.get_mut(y as usize) {
            if let Some(value) = row.get_mut(x as usize) {
                *value = *value + 1;
                if *value >= 9 {
                    coords.insert((x, y));
                    induce_neighbours((x, y), coords, octopii, checked)
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        }
    })
    .sum::<isize>()
}

fn process(input: &str) -> isize {
    let mut octopii = input
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect_vec())
        .collect_vec();

    let mut total_flashes = 0;
    let mut checked = vec![];

    for _ in 1..100 {
        octopii = octopii
            .into_iter()
            .map(|l| l.into_iter().map(|n| n + 1).collect_vec())
            .collect_vec();

        for (x, y) in &checked {
            if octopii[*y as usize][*x as usize] >= 9 {
                octopii[*y as usize][*x as usize] = 0;
            }
        }

        checked.drain(..);

        let mut points = (0..octopii[0].len())
            .cartesian_product(0..octopii.len())
            .filter(|&(x, y)| octopii[y][x] == 9)
            .map(|(x, y)| (x as isize, y as isize))
            .collect::<HashSet<(isize, isize)>>();

        while let Some(&p) = points.iter().next() {
            total_flashes += induce_neighbours(p, &mut points, &mut octopii, &mut checked);
        }
    }

    total_flashes
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
