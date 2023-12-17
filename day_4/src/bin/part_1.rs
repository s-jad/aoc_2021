use itertools::Itertools;
use std::time::Instant;

fn check_win(card: &mut Vec<Vec<(usize, bool)>>) -> bool {
    let mut row_win_check = Vec::new();
    let mut col_win_check = vec![Vec::new(); card[0].len()];
    let mut win = false;

    for i in 0..card.len() {
        for j in 0..card[0].len() {
            if card[i][j].1 == true {
                row_win_check.push(card[i][j].0);
                col_win_check[j].push(card[i][j].0);

                if row_win_check.len() == 5 || col_win_check[j].len() == 5 {
                    win = true;
                }
            } else {
                row_win_check.drain(..);
                col_win_check[j].clear();
            }
        }
    }

    win
}

fn process(input: &str) -> usize {
    let parts = input.split_terminator("\n\n").collect_vec();
    let mut nums = parts[0]
        .split_terminator(",")
        .map(|n| n.parse::<usize>().expect("should be number"))
        .collect_vec()
        .into_iter();

    let mut cards = parts[1..]
        .into_iter()
        .map(|card| {
            card.split_whitespace()
                .map(|n| (n.parse::<usize>().expect("should be number"), false))
                .collect_vec()
                .chunks(5)
                .map(|c| c.to_vec())
                .collect_vec()
        })
        .collect_vec();

    const CARD_WIDTH: usize = 5;

    let mut score = 0;

    while score == 0 {
        let num = nums.next().unwrap();

        for card in cards.iter_mut() {
            for row in card.iter_mut() {
                if let Some(pos) = row.iter().position(|&x| x == (num, false)) {
                    row[pos].1 = true;
                }
            }

            let win = check_win(card);

            if win {
                score = card
                    .iter()
                    .map(|row| {
                        row.iter()
                            .filter_map(|&(n, b)| if b == false { Some(n) } else { None })
                            .sum::<usize>()
                    })
                    .sum::<usize>()
                    * num;
            }
        }
    }

    score
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
