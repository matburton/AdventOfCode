
const EXAMPLE: &str =
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

const INPUT: &str = include_str!("../input/day_4.txt");

use std::collections::BTreeSet;

fn parse_numbers(text: &str) -> BTreeSet<usize> {

    text.split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect()
}

fn win_counts(input: &str) -> Vec<usize> {

    input.split('\n')
         .map(|l| l.split(':').skip(1).next().unwrap())
         .map(|l| l.split('|').map(parse_numbers).collect())
         .map(|n: Vec<_>| n[1].intersection(&n[0]).count())
         .collect()
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {
    
        win_counts(input).into_iter()
                         .map(|c| match c { 0 => 0, _ => 1 << (c - 1) })
                         .sum()
    }
    
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 13); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 24160); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> usize {
    
        let win_counts = win_counts(input);

        let mut queue = (0 .. win_counts.len()).collect::<Vec<_>>();

        let mut card_count = win_counts.len();

        while let Some(index) = queue.pop() {

            for offset in 0 .. win_counts[index] {

                queue.push(index + 1 + offset);

                card_count += 1;
            }
        }

        card_count
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 30); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 5659035); }
}