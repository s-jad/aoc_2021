use itertools::Itertools;
use std::time::Instant;

fn check_win(card: &mut Vec<Vec<(usize, bool)>>) -> bool {
    let mut row_win_check = Vec::new();
    let mut col_win_check = vec![Vec::new(); card[0].len()];
    let mut win = false;

    for i in 0..card.len() {
        for j in 0..card[0].len() {
            if card[i][j].1 == true {
                col_win_check[j].push(card[i][j].0);

                if col_win_check[j].len() == 5 {
                    println!("col_win_check => {:?}", col_win_check);
                    win = true;
                }
            } else {
                col_win_check[j].clear();
            }
            if card[i].iter().all(|(_, b)| b == &true) {
                println!("row_win_check => {:?}", row_win_check);
                row_win_check.push(card[i][j].0);
                win = true;
            } else {
                row_win_check.drain(..);
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
            (
                card.split_whitespace()
                    .map(|n| (n.parse::<usize>().expect("should be number"), false))
                    .collect_vec()
                    .chunks(5)
                    .map(|c| c.to_vec())
                    .collect_vec(),
                false,
            )
        })
        .collect_vec();

    let mut remaining_cards = cards.len();
    let mut score = 0;

    while score == 0 {
        let num = nums.next().unwrap();

        for card in cards.iter_mut().filter(|c| c.1 == false) {
            for row in card.0.iter_mut() {
                if let Some(pos) = row.iter().position(|&x| x == (num, false)) {
                    row[pos].1 = true;
                }
            }

            let win = check_win(&mut card.0);

            if win {
                remaining_cards -= 1;

                if remaining_cards >= 1 {
                    println!("card being taken out => {:?}\n", card);
                    card.1 = true;
                }

                if remaining_cards == 0 {
                    println!("last card to win => {:?}", card);
                    println!("num => {:?}", num);
                    let sum = card
                        .0
                        .iter()
                        .map(|row| {
                            row.iter()
                                .filter_map(|&(n, b)| if b == false { Some(n) } else { None })
                                .sum::<usize>()
                        })
                        .sum::<usize>();

                    println!("sum => {:?}", sum);
                    score = sum * num;
                }
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
