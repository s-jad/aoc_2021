use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn induce_neighbours(
    (x, y): (isize, isize),
    coords: &mut HashSet<(isize, isize)>,
    octopii: &mut [[u8; 10]; 10],
    flashed: &mut HashSet<(isize, isize)>,
) -> isize {
    if !coords.remove(&(x, y)) || flashed.contains(&(x, y)) {
        return 0;
    }

    if octopii[y as usize][x as usize] >= 9 {
        flashed.insert((x, y));
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
        if x >= 0 && x < 10 && y >= 0 && y < 10 {
            octopii[y as usize][x as usize] += 1;
            if octopii[y as usize][x as usize] >= 9 {
                coords.insert((x, y));
                induce_neighbours((x, y), coords, octopii, flashed)
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
    let mut octopii: [[u8; 10]; 10] = input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|c| c - b'0')
                .collect_vec()
                .as_slice()
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .as_slice()
        .try_into()
        .unwrap();

    let mut total_flashes = 0;
    let mut flashed = HashSet::new();

    for _ in 1..100 {
        for (x, y) in (0..10).cartesian_product(0..10) {
            octopii[y][x] += 1;
        }

        for (x, y) in &flashed {
            octopii[*y as usize][*x as usize] = 0;
        }

        flashed.drain();

        let mut points = (0..octopii[0].len())
            .cartesian_product(0..octopii.len())
            .filter(|&(x, y)| octopii[y][x] == 9)
            .map(|(x, y)| (x as isize, y as isize))
            .collect::<HashSet<(isize, isize)>>();

        while let Some(&p) = points.iter().next() {
            total_flashes += induce_neighbours(p, &mut points, &mut octopii, &mut flashed);
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
