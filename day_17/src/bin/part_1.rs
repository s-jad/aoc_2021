use itertools::Itertools;
use std::{ops::Range, time::Instant};

#[derive(Debug)]
struct Area {
    x: Range<isize>,
    y: Range<isize>,
}

impl Area {
    fn new(x: Range<isize>, y: Range<isize>) -> Self {
        Self { x, y }
    }

    fn hit(&self, x: isize, y: isize) -> bool {
        if self.x.contains(&x) && self.y.contains(&y) {
            return true;
        }
        return false;
    }
}

fn process(input: &str) -> isize {
    let target = Area::new(241..273, -97..-63);

    let mut largest_y = 0;
    let start = (0, 0);
    let min_x = 241f32;
    let max_x = 273f32;

    let start_x = ((2f32 * min_x).sqrt() - 1f32) as isize;
    let end_x = ((2f32 * max_x).sqrt() - 1f32) as isize;

    for y in 75..100 {
        let mut y_path = Vec::new();

        for x in start_x..=(end_x + 1) {
            let (mut current_x, mut current_y) = start;
            let mut vy = y;
            let mut vx = x;

            while current_x < 273 && current_y > -97 {
                current_y = current_y + vy;
                current_x = current_x + vx;
                y_path.push(current_y);

                if vx > 0 {
                    vx -= 1;
                } else if vx < 0 {
                    vx += 1;
                }
                vy -= 1;

                if (&target).hit(current_x, current_y) {
                    let path_max = y_path.iter().max().unwrap();
                    if largest_y < *path_max {
                        largest_y = *path_max;
                    }
                }
            }
        }
    }

    largest_y
}

fn main() {
    let input = include_str!("../../test.txt");

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
