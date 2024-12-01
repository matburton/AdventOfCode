
const EXAMPLE: &str = "32T3K 765\n\
                       T55J5 684\n\
                       KK677 28\n\
                       KTJJT 220\n\
                       QQQJA 483";

const INPUT: &str = include_str!("../input/day_7.txt");

use std::{ cmp::Ordering, collections::BTreeMap };

type Hand = [char; 5];

const CARD_ORDER: [char; 14] =
    ['*', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

fn card_rank(card: char) -> usize {

    CARD_ORDER.iter().enumerate().find(|(_, &c)| c == card).unwrap().0
}

fn hand_type(hand: &Hand) -> usize {

    let mut map = BTreeMap::new();

    for char in hand  {
        
        if let Some(count) = map.get_mut(char) {

            *count += 1;
        }
        else { map.insert(*char, 1); }
    }

    let wild_count = map.remove(&'*').unwrap_or(0);

    if wild_count == 5 { return 6; }

    let mut counts = map.values().cloned().collect::<Vec<_>>();

    counts.sort();
    counts.reverse();

    counts[0] += wild_count;

    match counts[..] { [5]        => 6,
                       [4, 1]     => 5,
                       [3, 2]     => 4,
                       [3, ..]    => 3,
                       [2, 2, ..] => 2,
                       [2, ..]    => 1,
                       _          => 0 }
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {

    let ordering = hand_type(a).cmp(&hand_type(b));

    if ordering != Ordering::Equal { return ordering; }

    a.iter()
     .zip(b)
     .map(|(card_a, card_b)| card_rank(*card_a).cmp(&card_rank(*card_b)))
     .find(|&o| o != Ordering::Equal)
     .unwrap()
}

fn parse_hand(text: &str) -> Hand {

    text.chars().collect::<Vec<_>>().try_into().unwrap()
}

fn get_result(input: &str) -> usize {

    let mut hand_bids =
        input.split('\n')
             .map(|l| l.split(' ').collect::<Vec<_>>())
             .map(|a| (parse_hand(a[0]), a[1].parse::<usize>().unwrap()))
             .collect::<Vec<_>>();

    hand_bids.sort_by(|(a, _), (b, _)| compare_hands(a, b));

    hand_bids.iter()
             .enumerate()
             .map(|(index, (_, bid))| (index + 1) * bid)
             .sum()
}

mod part_1 {

    use super::*;

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 6440); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 247815719); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> usize {

        super::get_result(&input.replace('J', "*"))
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 5905); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 248747492); }
}