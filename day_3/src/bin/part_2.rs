use itertools::Itertools;
use std::time::Instant;

fn array_to_binary_num(array: &[u8]) -> u16 {
    let mut num = 0;
    for (i, &bit) in array.iter().rev().enumerate() {
        num |= (bit as u16) << i;
    }
    num
}

fn process(input: &str) -> usize {
    let bits = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(2).expect("digit") as u8)
                .collect_vec()
        })
        .collect_vec();

    let mut oxygen: Vec<u8> = Vec::new();
    let mut scrubber: Vec<u8> = Vec::new();
    let mut remaining_o = bits.clone();
    let mut remaining_s = bits.clone();

    for j in 0..bits[0].len() {
        let mut one_count_o = 0;
        let mut zero_count_o = 0;

        let mut one_count_s = 0;
        let mut zero_count_s = 0;

        if remaining_s.len() == 1 {
            scrubber = remaining_s[0].clone();
        }

        for i in 0..remaining_o.len() {
            if remaining_o[i][j] == 1 {
                one_count_o += 1;
            } else {
                zero_count_o += 1;
            }
        }

        for i in 0..remaining_s.len() {
            if remaining_s[i][j] == 1 {
                one_count_s += 1;
            } else {
                zero_count_s += 1;
            }
        }

        if one_count_o < zero_count_o {
            remaining_o = remaining_o.into_iter().filter(|b| b[j] == 0).collect_vec();
            oxygen.push(0);
        } else {
            remaining_o = remaining_o.into_iter().filter(|b| b[j] == 1).collect_vec();
            oxygen.push(1);
        }

        if one_count_s < zero_count_s {
            remaining_s = remaining_s.into_iter().filter(|b| b[j] == 1).collect_vec();
        } else {
            remaining_s = remaining_s.into_iter().filter(|b| b[j] == 0).collect_vec();
        }
    }

    let o_rating = array_to_binary_num(&oxygen);
    let s_rating = array_to_binary_num(&scrubber);

    o_rating as usize * s_rating as usize
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
