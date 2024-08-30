
const INPUT: &str = include_str!("../input/day_15.txt");

const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

fn get_hash(text: &str) -> usize {

    text.chars().fold(0, |s, c| (s + c as usize) * 17 % 256)
}

#[test]
fn test_get_hash() { assert_eq!(get_hash("HASH"), 52); }

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize { input.split(',').map(get_hash).sum() }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 1320); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 511215); }
}