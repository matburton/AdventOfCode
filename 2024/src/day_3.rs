
const INPUT: &str = include_str!("../input/day_3.txt");

use regex::{ Match, Regex };

fn parse(capture: Option<Match>) -> usize {

    capture.unwrap().as_str().parse::<usize>().unwrap()
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        Regex::new(r"mul\((\d+),(\d+)\)")
            .unwrap()
            .captures_iter(input)
            .map(|m| parse(m.get(1)) * parse(m.get(2)))
            .sum()
    }

    const EXAMPLE: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
   
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 161); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 183669043); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let regex = Regex::new(r"do\(\)|don\'t\(\)|mul\((\d+),(\d+)\)").unwrap();

        let (mut enabled, mut sum) = (true, 0);

        for regex_match in regex.captures_iter(input) {

            sum += match regex_match.get(0).unwrap().as_str() {

                "do()"    => { enabled = true;  0 },
                "don't()" => { enabled = false; 0 },
                _ if !enabled => 0,
                _ => parse(regex_match.get(1)) * parse(regex_match.get(2))
            }
        }

        sum
    }

    const EXAMPLE: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
   
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 48); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 59097164); }
}