
const EXAMPLE: &str = "0 3 6 9 12 15\n\
                       1 3 6 10 15 21\n\
                       10 13 16 21 30 45";

const INPUT: &str = include_str!("../input/day_9.txt");

fn diffs(line: &str) -> Vec<Vec<isize>> {

    let mut diffs = Vec::from
        ([line.split_whitespace()
              .map(|t| t.parse::<isize>().unwrap())
              .collect::<Vec<_>>()]);
    loop {

        let last = diffs.last().unwrap();

        let next = last.iter()
                       .zip(last.iter().skip(1))
                       .map(|(a, b)| b - a)
                       .collect::<Vec<_>>();

        if next.iter().all(|&v| v == 0) { return diffs; }

        diffs.push(next);
    }
}

mod part_1 {

    use super::*;

    fn next_val(line: &str) -> isize {

        diffs(line).iter().rev().fold(0, |a, d| a + d.last().unwrap())
    }

    fn get_result(input: &str) -> isize {

        input.split('\n').map(next_val).sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 114); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1930746032); }
}

mod part_2 {

    use super::*;

    fn next_val(line: &str) -> isize {

        diffs(line).iter().rev().fold(0, |a, d| d.first().unwrap() - a)
    }

    fn get_result(input: &str) -> isize {

        input.split('\n').map(next_val).sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 2); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1154); }
}