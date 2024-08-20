
const INPUT: &str = include_str!("../input/day_1.txt");

mod part_1 {

    use super::*;

    fn first_digit(iterator: impl IntoIterator<Item = char>) -> u32 {

        iterator.into_iter()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_string()
                .parse()
                .unwrap()
    }
    
    fn get_result(input: &str) -> u32 {
    
        input.split('\n')
             .map(|l| (first_digit(l.chars()),
                       first_digit(l.chars().rev())))
             .map(|t| 10 * t.0 + t.1)
             .sum()
    }
    
    #[test]
    fn example() {
    
        const example_input: &str = "1abc2\n\
                                     pqr3stu8vwx\n\
                                     a1b2c3d4e5f\n\
                                     treb7uchet";
    
        assert_eq!(get_result(example_input), 142);
    }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 56397); }
}

mod part_2 {

    use super::*;

    use std::sync::LazyLock;

    use regex::Regex;

    const DIGIT_WORDS: [&str; 10] = ["zero",
                                     "one",
                                     "two",
                                     "three",
                                     "four",
                                     "five",
                                     "six",
                                     "seven",
                                     "eight",
                                     "nine"];

    static REGEX_FROM_LEFT: LazyLock<Regex> = LazyLock::new(|| {

        Regex::new(&format!("({}|[0-9])", DIGIT_WORDS.join("|"))).unwrap()
    });

    static REGEX_FROM_RIGHT: LazyLock<Regex> = LazyLock::new(|| {

        Regex::new(&format!(".*({}|[0-9])", DIGIT_WORDS.join("|"))).unwrap()
    });

    fn first_digit(line: &str, regex: &Regex) -> usize {

        let capture = &regex.captures_iter(line).next().unwrap()[1];

        match DIGIT_WORDS.into_iter().position(|s| s == capture) {

            Some(index) => index,

            _ => capture.parse().unwrap()
        }
    }
    
    fn get_result(input: &str) -> usize {
    
        input.split('\n')
             .map(|l| (first_digit(l, &*REGEX_FROM_LEFT),
                       first_digit(l, &*REGEX_FROM_RIGHT)))
             .map(|t| 10 * t.0 + t.1)
             .sum()
    }

    #[test]
    fn example() {
    
        const example_input: &str = "two1nine\n\
                                     eightwothree\n\
                                     abcone2threexyz\n\
                                     xtwone3four\n\
                                     4nineeightseven2\n\
                                     zoneight234\n\
                                     7pqrstsixteen";
    
        assert_eq!(get_result(example_input), 281);
    }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 55701); }
}