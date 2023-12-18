use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut fishies = input
        .split_terminator(&[',', '\n'][..])
        .map(|s| s.parse::<i8>().expect("number"))
        .collect_vec();
    let mut new_fish: Vec<u8> = Vec::new();

    let mut day_count = 80;

    while day_count != 0 {
        for fish in fishies.iter_mut() {
            *fish = (*fish - 1).rem_euclid(7);
            if fish == &0 {
                new_fish.push(10);
            }
        }

        let new_fish_len = new_fish.len();
        for idx in 0..new_fish_len {
            new_fish[idx] = new_fish[idx] - 1;

            if new_fish[idx] == 6 {
                fishies.push(6);
            }
        }
        new_fish = new_fish.into_iter().filter(|f| f > &6).collect_vec();

        day_count -= 1;
    }

    fishies.len() + new_fish.iter().filter(|f| f < &&9).count()
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
