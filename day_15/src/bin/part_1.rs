use itertools::Itertools;
use std::{collections::BinaryHeap, time::Instant};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl State {
    fn new(cost: usize, position: (usize, usize)) -> Self {
        Self { cost, position }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // other first, not self to create MinHeap, default = MaxHeap
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // other first, not self to create MinHeap, default = MaxHeap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn a_star(grid: &Vec<Vec<State>>, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    heap.push(State::new(0, start));

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        };

        if visited[position.1][position.0] {
            continue;
        } else {
            visited[position.1][position.0] = true;
        }

        let &State {
            position: (x, y), ..
        } = &State {
            position,
            ..State::default()
        };
        let neighbours = [
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
            (x + 1, y),
            (x, y + 1),
        ];

        for (nx, ny) in neighbours {
            let mut lowest_cost_move = std::usize::MAX;
            if nx <= goal.0 && ny <= goal.1 && !visited[ny][nx] {
                let next_cost = grid[ny][nx].cost;

                if next_cost < lowest_cost_move {
                    lowest_cost_move = next_cost;
                    heap.push(State::new(next_cost + cost, (nx, ny)));
                }
            }
        }
    }

    None
}

fn process(input: &str) -> usize {
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    let num = c.to_digit(10).unwrap() as usize;
                    State::new(num, (x, y))
                })
                .collect_vec()
        })
        .collect_vec();

    let start: (usize, usize) = (0, 0);
    let goal = (grid[0].len() - 1, grid.len() - 1);

    match a_star(&grid, start, goal) {
        Some(path) => return path,
        None => return 0,
    }
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
