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

fn process() -> usize {
    let target = Area::new(241..274, -97..-62);

    let start = (0, 0);
    let max_x = 274;
    let min_x = 241;

    let start_x = ((2f32 * min_x as f32).sqrt() - 1f32) as isize;
    let mut valid = Vec::new();

    for y in -98..98 {
        for x in start_x..=max_x {
            let (mut current_x, mut current_y) = start;
            let mut vy = y;
            let mut vx = x;

            while current_x < max_x && current_y > -98 {
                current_y = current_y + vy;
                current_x = current_x + vx;

                if vx > 0 {
                    vx -= 1;
                } else if vx < 0 {
                    vx += 1;
                }
                vy -= 1;

                if (&target).hit(current_x, current_y) {
                    valid.push((x, y));
                    break;
                }
            }
        }
    }

    valid.len()
}

fn main() {
    let start = Instant::now();
    let output = process();
    let time = start.elapsed();

    dbg!(output, time);
}
