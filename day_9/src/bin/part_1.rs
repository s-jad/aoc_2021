use itertools::Itertools;
use std::time::Instant;

fn get_neighbour(grid: &Vec<Vec<usize>>, x: usize, y: usize) -> Vec<Option<usize>> {
    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, 1, 0, -1];

    let mut neighbours = Vec::new();

    for i in 0..4 {
        let nx = x as i32 + dx[i];
        let ny = y as i32 + dy[i];

        if nx >= 0 && ny >= 0 && ny < grid[0].len() as i32 && nx < grid.len() as i32 {
            neighbours.push(Some(grid[nx as usize][ny as usize]));
        } else {
            neighbours.push(None);
        }
    }

    neighbours
}

fn process(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(move |l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("should be digit") as usize)
                .collect_vec()
        })
        .collect_vec();

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
                local_minimums.push((grid[x][y], x, y));
            }
        }
    }

    local_minimums.iter().map(|(i, _, _)| i + 1).sum()
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
