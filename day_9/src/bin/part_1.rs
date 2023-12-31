use itertools::Itertools;
use std::time::Instant;

fn get_neighbour(grid: &[[usize; 100]; 100], x: usize, y: usize) -> [Option<usize>; 4] {
    let dx = [-1, 0, 1, 0];
    let dy = [0, 1, 0, -1];

    let mut neighbours = [Option::None; 4];

    for i in 0..4 {
        let nx = x as i32 + dx[i];
        let ny = y as i32 + dy[i];

        if nx >= 0 && ny >= 0 && ny < grid[0].len() as i32 && nx < grid.len() as i32 {
            neighbours[i] = Some(grid[nx as usize][ny as usize]);
        }
    }

    neighbours
}

fn process(input: &str) -> usize {
    let grid: [[usize; 100]; 100] = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("should be digit") as usize)
                .collect::<Vec<usize>>()
                .as_slice()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[usize; 100]>>()
        .as_slice()
        .try_into()
        .unwrap();

    let mut local_minimums = Vec::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let neighbours = get_neighbour(&grid, x, y);
            let mut local_min = true;
            for neighbour in neighbours {
                if grid[x][y] >= neighbour.unwrap_or(1000) {
                    local_min = false;
                }
            }

            if local_min {
                local_minimums.push(grid[x][y]);
            }
        }
    }

    local_minimums.iter().map(|i| i + 1).sum()
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
