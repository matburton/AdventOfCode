
const EXAMPLE: &str = "Time:      7  15   30\n\
                       Distance:  9  40  200";

const INPUT: &str = include_str!("../input/day_6.txt");

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        0
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 288); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 0); }
}